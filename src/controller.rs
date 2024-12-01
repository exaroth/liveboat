use std::cell::RefCell;
use std::error::Error;
use std::sync::Arc;

use rusqlite::Error as SQLiteError;
use rusqlite::{params_from_iter, Connection, Result, Rows};

use crate::args::Args;
use crate::builder::Builder;
use crate::cli;
use crate::errors::FilesystemError;
use crate::feed::Feed;
use crate::feed_item::FeedItem;
use crate::opts::Options;
use crate::paths::Paths;
use crate::template::Context;
use crate::urls::UrlReader;

#[derive(Debug)]
pub struct Controller {
    paths: Paths,
    options: Options,
    url_reader: UrlReader,
}

const FEED_ITEMS_SQL: &str = "SELECT 
    feed.rssurl AS feed_url,
    feed.title AS feed_title,
    items.title AS item_title,
    items.url AS item_url,
    items.author AS item_author,
    items.enclosure_description AS item_desc,
    items.pubDate AS pub_date,
    items.unread AS unread,
    items.content AS content,
    items.id AS guid,
    items.enclosure_url AS enc_url,
    items.enclosure_description_mime_type AS enc_mime_type,
    items.flags AS flags
FROM rss_item AS items
JOIN rss_feed AS feed ON feed.rssurl = items.feedurl
WHERE datetime(items.pubDate, 'unixepoch') >= datetime('now', '-10 days')
AND items.deleted=0;
";

impl Controller {
    pub fn init(args: &Args) -> Result<Controller, Box<dyn Error>> {
        let paths = Paths::new(&args.config_file)?;
        if !paths.initialized() {
            Err(FilesystemError::NotInitialized)?;
        }
        let opts = Options::init(paths.config_file())?;
        let url_reader = UrlReader::init(paths.url_file());
        // TODO: verify
        // set config path
        // check for config path
        let ctrl = Controller {
            paths: paths,
            options: opts,
            url_reader: url_reader,
        };
        Ok(ctrl)
    }

    pub fn cold_start(args: &Args) -> Result<(), Box<dyn Error>> {
        let mut paths = Paths::new(&args.config_file)?;
        paths.update_with_args(args)?;
        let mut opts = Options::default();
        opts.title = cli::prompt_string(opts.title, "Enter your feed page title:")?;
        opts.newsboat_urls_file =
            cli::prompt_path(paths.url_file(), true, "Enter path to Newsboat urls file:")?;
        opts.newsboat_cache_file = cli::prompt_path(
            paths.cache_file(),
            true,
            "Enter path to Newsboat cache db file:",
        )?;
        opts.time_threshold = cli::prompt_int(
            opts.time_threshold,
            "Enter number of days in the past Liveboat should generate feeds for",
        )?;
        opts.show_read_articles = cli::confirm(
            opts.show_read_articles,
            "Should Liveboat include read articles in the feeds?",
        );
        opts.build_dir = cli::prompt_path(
            paths.build_dir(),
            false,
            "Where should Liveboat save generated pages to?",
        )?;
        Ok(())
    }

    pub fn process_feeds(&self) -> Result<(), Box<dyn Error>> {
        let feed_items = self.get_feed_item_data()?;
        let feeds = self.get_url_feeds()?;
        self.populate_url_feeds(&feeds, &feed_items);
        let q_feeds = self.get_query_feeds(&feeds)?;
        let ctx = Context::init(&feeds, &q_feeds, &self.options);
        let builder = Builder::init(
            self.paths.tmp_dir(),
            self.paths.build_dir(),
            self.paths.template_path(),
            self.options.template_name(),
            ctx,
        )?;
        builder.create_tmp();

        self.save_json_feeds(&builder, &q_feeds, &feeds)?;
        builder.render_template()?;

        builder.copy_data()?;
        builder.clean_up();
        Ok(())
    }

    /// Generate json files for each feed.
    fn save_json_feeds(
        &self,
        builder: &Builder<Context>,
        query_feeds: &Vec<Feed>,
        url_feeds: &Vec<Arc<RefCell<Feed>>>,
    ) -> Result<(), Box<dyn Error>> {
        for f in query_feeds {
            self.save_json_feed(builder, f)?;
        }
        for f in url_feeds {
            self.save_json_feed(&builder, &f.borrow())?;
        }
        Ok(())
    }

    fn save_json_feed(
        &self,
        builder: &Builder<Context>,
        feed: &Feed,
    ) -> Result<(), Box<dyn Error>> {
        if feed.is_empty() || feed.is_hidden() {
            return Ok(());
        }
        // TODO, if debug = true use pretty output
        let data = serde_json::to_string_pretty(&feed)?;
        builder.save_feed_data(feed.id(), data.as_bytes())?;
        Ok(())
    }

    fn populate_url_feeds(&self, feeds: &Vec<Arc<RefCell<Feed>>>, feed_items: &Vec<FeedItem>) {
        for item in feed_items {
            if let Some(f) = feeds.iter().find(|f| f.borrow().url() == item.feed_url()) {
                let mut i = item.clone();
                i.set_ptr(Arc::clone(f));
                f.borrow_mut().add_item(i);
                continue;
            }
        }
        for f in feeds {
            f.borrow_mut().sort_items()
        }
    }

    fn get_url_feeds(&self) -> Result<Vec<Arc<RefCell<Feed>>>, Box<dyn Error>> {
        let url_feeds = self.url_reader.get_url_feeds();
        let urls = url_feeds.iter().map(|u| u.url.clone()).collect();
        let feed_data = self.get_feed_data(urls)?;
        for f in &feed_data {
            if let Some(url_feed) = url_feeds.iter().find(|u| &u.url == f.borrow().url()) {
                f.borrow_mut().update_with_url_data(
                    url_feed.tags.clone(),
                    url_feed.hidden,
                    url_feed.title_override.clone(),
                );
            }
        }
        Ok(feed_data)
    }

    fn get_feed_item_data(&self) -> Result<Vec<FeedItem>, Box<dyn Error>> {
        let conn = &self.get_db_connection()?;
        let mut stmt = conn.prepare(FEED_ITEMS_SQL)?;
        let mut r = stmt.query([])?;
        let results = self.load_feed_items(&mut r)?;
        Ok(results)
    }

    fn get_query_feeds(
        &self,
        feeds: &Vec<Arc<RefCell<Feed>>>,
    ) -> Result<Vec<Feed>, Box<dyn Error>> {
        let mut result = Vec::new();
        let query_feeds = &self.url_reader.get_query_urls()?;
        for query_f in query_feeds {
            let mut q = Feed::init_query_feed(query_f.title.clone());
            for f in feeds {
                for i in &f.borrow().items {
                    match query_f.matcher.matches(i) {
                        // TODO, check MatcherError
                        Err(_) => continue,
                        Ok(matches) => {
                            if matches {
                                q.add_item(i.clone())
                            }
                        }
                    }
                }
            }
            q.sort_items();
            result.push(q)
        }
        Ok(result)
    }

    fn get_db_connection(&self) -> Result<Connection, SQLiteError> {
        let conn = Connection::open(&self.paths.cache_file())?;
        Ok(conn)
    }

    fn load_feed_items(&self, rows: &mut Rows<'_>) -> Result<Vec<FeedItem>, SQLiteError> {
        let mut results: Vec<FeedItem> = Vec::new();
        while let Some(row) = rows.next()? {
            let feed_item = FeedItem::from_db_row(row)?;
            results.push(feed_item);
        }
        Ok(results)
    }

    fn get_feed_data(&self, urls: Vec<String>) -> Result<Vec<Arc<RefCell<Feed>>>, SQLiteError> {
        let repeat_vars = |c| {
            assert_ne!(c, 0);
            let mut s = "?,".repeat(c);
            // Remove trailing comma
            s.pop();
            s
        };
        let sql = format!(
            "SELECT rssurl, title, url FROM rss_feed where rssurl in ({});",
            repeat_vars(urls.len())
        );
        let conn = &self.get_db_connection()?;
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params_from_iter(urls.iter()), |row| {
            let f = Feed::init(row.get(0)?, row.get(1)?, row.get(2)?);
            Ok(f)
        })?;
        let mut result = Vec::new();
        for r in rows {
            match r {
                Ok(f) => result.push(Arc::new(RefCell::new(f))),
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }
}
