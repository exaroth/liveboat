use log::{info, trace, warn};
use std::cell::RefCell;
use std::error::Error;
use std::fs::read_to_string;
use std::sync::Arc;

use crate::args::Args;
use crate::builder::SinglePageBuilder;
use crate::db::{get_feed_data, get_feed_item_data};
use crate::errors::FilesystemError;
use crate::feed::{Feed, FeedList};
use crate::feed_item::FeedItem;
use crate::opts::Options;
use crate::paths::Paths;
use crate::template::Context;
use crate::urls::UrlReader;

/// Build controller faciliates the process of parsing url
/// files, retrieving feed information from db, generating feed objects
/// and building output files.
#[derive(Debug)]
pub struct BuildController {
    paths: Paths,
    options: Options,
    url_reader: UrlReader,
    debug: bool,
}

impl BuildController {
    pub fn init(args: &Args) -> Result<BuildController, Box<dyn Error>> {
        info!("Initializing controller");
        let mut paths = Paths::new(&args.config_file)?;
        info!("Default paths are {}", paths);
        if !paths.initialized() {
            Err(FilesystemError::NotInitialized)?;
        }
        let opts = Options::init(paths.config_file())?;
        info!("Opts are {}", opts);
        paths.update_with_opts(
            &opts.newsboat_urls_file,
            &opts.newsboat_cache_file,
            &opts.build_dir,
            &opts.template_name(),
        );
        info!("Paths after opt update {}", paths);
        paths.update_with_args(&args)?;
        info!("Paths after arg update {}", paths);
        paths.check_all()?;
        let url_file = read_to_string(paths.url_file())?;
        let url_reader = UrlReader::init(url_file);
        let ctrl = BuildController {
            paths: paths,
            options: opts,
            url_reader: url_reader,
            debug: args.debug,
        };
        info!("Controller initialized");
        Ok(ctrl)
    }

    /// Main template method used for processing build command.
    /// We first retrieve feed and article information from database
    /// as well as parse urls file then create feed and query feed objects
    /// using matching rules provided by the latter and populate feed objects
    /// with the articles. Finally we utilize builder module to first output
    /// all the static page data to tmp dir and copy it to build directory.
    pub fn build(&self) -> Result<(), Box<dyn Error>> {
        info!("Processing feeds");
        let feed_items = get_feed_item_data(self.paths.cache_file(), self.options.time_threshold)?;
        let feeds = self.get_url_feeds()?;
        self.populate_url_feeds(&feeds, &feed_items);
        let q_feeds = self.get_query_feeds(&feeds)?;
        let ctx = Context::init(&feeds, &q_feeds, &self.options);
        let builder = SinglePageBuilder::init(
            self.paths.tmp_dir(),
            self.paths.build_dir(),
            self.paths.template_path(),
            self.options.template_name(),
            ctx,
        )?;
        builder.create_tmp()?;

        self.save_json_feeds(&builder, &q_feeds, &feeds)?;
        builder.render_template()?;

        builder.copy_data()?;
        builder.clean_up();
        println!(
            "Liveboat build saved to {}",
            self.paths.build_dir().display()
        );
        Ok(())
    }

    /// Generate json files for each feed.
    fn save_json_feeds(
        &self,
        builder: &SinglePageBuilder<Context>,
        query_feeds: &Vec<Feed>,
        url_feeds: &Vec<Arc<RefCell<Feed>>>,
    ) -> Result<(), Box<dyn Error>> {
        for f in query_feeds {
            self.save_json_feed(builder, f)?;
        }
        let q_list = FeedList::from_vec(query_feeds.clone());
        self.save_json_feedlist(&builder, &q_list, String::from("query_feeds"))?;
        let mut f_list = FeedList::new();
        for f in url_feeds {
            let feed = f.borrow();
            self.save_json_feed(&builder, &feed)?;
            if !feed.is_empty() && !feed.is_hidden() {
                f_list.add_feed(&feed);
            }
        }
        self.save_json_feedlist(&builder, &f_list, String::from("feeds"))?;
        Ok(())
    }
    fn save_json_feedlist(
        &self,
        builder: &SinglePageBuilder<Context>,
        feedlist: &FeedList,
        name: String,
    ) -> Result<(), Box<dyn Error>> {
        if self.debug {
            builder.save_feed_data(&name, serde_json::to_string_pretty(&feedlist)?.as_bytes())?;
        } else {
            builder.save_feed_data(&name, serde_json::to_string(&feedlist)?.as_bytes())?;
        }
        Ok(())
    }

    fn save_json_feed(
        &self,
        builder: &SinglePageBuilder<Context>,
        feed: &Feed,
    ) -> Result<(), Box<dyn Error>> {
        if feed.is_empty() || feed.is_hidden() {
            info!("Skipping saving feed: {:?}", feed);
            return Ok(());
        }
        if self.debug {
            builder.save_feed_data(feed.id(), serde_json::to_string_pretty(&feed)?.as_bytes())?;
        } else {
            builder.save_feed_data(feed.id(), serde_json::to_string(&feed)?.as_bytes())?;
        }
        Ok(())
    }

    /// Populate feeds with article items, filter out read articles based on the opt value.
    fn populate_url_feeds(&self, feeds: &Vec<Arc<RefCell<Feed>>>, feed_items: &Vec<FeedItem>) {
        info!("Populating feeds with feed items");
        for item in feed_items {
            if let Some(f) = feeds.iter().find(|f| f.borrow().url() == item.feed_url()) {
                if self.options.show_read_articles == false && item.is_unread() == false {
                    info!("Skipping item: {}", item);
                    continue;
                }
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

    /// Retrieve article data from db and populate it with data from urls.
    fn get_url_feeds(&self) -> Result<Vec<Arc<RefCell<Feed>>>, Box<dyn Error>> {
        let url_feeds = self.url_reader.get_url_feeds();
        let urls = url_feeds.iter().map(|u| u.url.clone()).collect();
        trace!("List of urls to retrieve: {}", format!("{:?}", urls));
        let mut result = Vec::new();
        let feed_data = get_feed_data(self.paths.cache_file(), urls)?;
        for mut f in feed_data {
            if let Some(url_feed) = url_feeds.iter().find(|u| &u.url == f.url()) {
                f.update_with_url_data(
                    url_feed.tags.clone(),
                    url_feed.hidden,
                    url_feed.title_override.clone(),
                    url_feed.line_no,
                );
                result.push(Arc::new(RefCell::new(f)))
            }
        }
        Ok(result)
    }

    /// Process query feed objects as defined in urls file - this is done by matching
    /// rules for each article against those defined by the user, we generate feed object
    /// for each query feed marking it appropriately.
    fn get_query_feeds(
        &self,
        feeds: &Vec<Arc<RefCell<Feed>>>,
    ) -> Result<Vec<Feed>, Box<dyn Error>> {
        let mut result = Vec::new();
        let query_feeds = &self.url_reader.get_query_urls()?;
        for query_f in query_feeds {
            let mut q = Feed::init_query_feed(query_f.title.clone(), query_f.line_no);
            for f in feeds {
                for i in &f.borrow().items {
                    match query_f.matcher.matches(i) {
                        Err(e) => {
                            warn!("Matcher error: {:?}", e);
                            continue;
                        }
                        Ok(matches) => {
                            if matches {
                                trace!("Query {} matched against item {}]", query_f.title, i);
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
}
