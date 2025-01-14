#[cfg(test)]
#[allow(unused_imports)]
use mockall::Sequence;

use log::{info, trace, warn};
use std::cell::RefCell;
use std::fs::read_to_string;
use std::sync::Arc;
use std::io::{self, Write};

use anyhow::Result;

use crate::args::Args;
use crate::builders::aux::Builder;
use crate::builders::spa_builder::SinglePageBuilder;
use crate::content::process_article_content;
use crate::db::{Connector, DBConnector};
use crate::errors::FilesystemError;
use crate::feed::Feed;
use crate::feed_item::FeedItem;
use crate::opts::Options;
use crate::paths::Paths;
use crate::template::{SimpleContext, TemplateConfig};
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
    pub fn init(args: &Args) -> Result<BuildController> {
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
    pub fn build(&self) -> Result<()> {
        info!("Processing feeds");
        let db_connector = DBConnector::init(self.paths.cache_file())?;
        let feed_items = self.get_feed_items(&db_connector, self.options.time_threshold)?;
        let feeds = self.get_url_feeds(&db_connector)?;
        self.populate_url_feeds(&feeds, &feed_items);
        let q_feeds = self.get_query_feeds(&feeds)?;
        let tpl_config = TemplateConfig::get_config_for_template(self.paths.template_path())?;
        let ctx = SimpleContext::init(
            &feeds,
            &q_feeds,
            &self.options,
            &tpl_config.template_settings,
            tpl_config.version.clone(),
        );
        let builder = self.get_builder(&ctx)?;
        builder.create_tmp()?;
        builder.generate_aux_data()?;
        builder.render_templates()?;
        builder.copy_data()?;
        builder.clean_up();
        println!(
            "Liveboat feed page saved to {}",
            self.paths.build_dir().display()
        );
        Ok(())
    }

    /// Retrieve builder instance to be used for generating static content.
    /// At the moment there is only SPA builder implemented.
    fn get_builder<'a>(&'a self, context: &'a SimpleContext) -> Result<Box<dyn Builder + 'a>> {
        let simple_builder = SinglePageBuilder::init(
            self.paths.tmp_dir(),
            self.paths.build_dir(),
            self.paths.template_path(),
            context,
            self.debug,
        )?;
        return Ok(Box::new(simple_builder));
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
            f.borrow_mut().sort_items();
            println!(
                "Processing content for feed: {}, total items: {}",
                f.borrow().title(),
                f.borrow().truncated_items_count()
            );
            for item in f.borrow_mut().truncated_iter() {
                let res =
                    process_article_content(item.url(), &mut item.content().clone(), &self.options);
                if res.is_err() {
                    info!(
                        "Error processing content {}, {}",
                        item.content(),
                        res.unwrap_err()
                    );
                    item.set_content(String::new());
                    continue;
                }
                let (new_content, new_url, content_length, comments_url) = res.unwrap();
                item.set_content_length(content_length);
                item.set_content(new_content);
                item.set_url(new_url);
                if comments_url.is_some() {
                    item.set_comments_url(comments_url.unwrap())
                }
            }
        }
    }

    /// Retrieve article data from db and populate it with data from urls.
    fn get_url_feeds(&self, db_connector: &impl Connector) -> Result<Vec<Arc<RefCell<Feed>>>> {
        let url_feeds = self.url_reader.get_url_feeds();
        let urls = url_feeds.iter().map(|u| u.url.clone()).collect();
        trace!("List of urls to retrieve: {}", format!("{:?}", urls));
        let mut result = Vec::new();
        let feed_data = db_connector.get_feeds(urls)?;
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
    fn get_query_feeds(&self, feeds: &Vec<Arc<RefCell<Feed>>>) -> Result<Vec<Feed>> {
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

    /// Retrieve feeds from cache db.
    fn get_feed_items(
        &self,
        db_connector: &impl Connector,
        days_back: u64,
    ) -> Result<Vec<FeedItem>> {
        let db_data = db_connector.get_feed_items(days_back)?;
        return Ok(db_data);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_processing_query_feeds_simple() {
        let contents = "
\"query:News:tags # \\\"news\\\"\"
            ";
        let reader = UrlReader::init(contents.to_string());
        let ctrl = BuildController {
            url_reader: reader,
            paths: Paths::default(),
            options: Options::default(),
            debug: false,
        };

        let f1 = Arc::new(RefCell::new(Feed::init(
            "".to_string(),
            "Feed1".to_string(),
            "".to_string(),
        )));

        f1.borrow_mut()
            .update_with_url_data(Vec::from(["news".to_string()]), false, None, 0);
        let mut item1 = FeedItem::new(
            "Feed1 Item1",
            "http://feed1.com",
            "",
            "",
            970000000,
            true,
            "",
            1,
        );
        item1.set_ptr(Arc::clone(&f1));
        f1.borrow_mut().add_item(item1);

        let f2 = Arc::new(RefCell::new(Feed::init(
            "".to_string(),
            "Feed2".to_string(),
            "".to_string(),
        )));

        f2.borrow_mut()
            .update_with_url_data(Vec::from(["other".to_string()]), false, None, 0);
        let mut item2 = FeedItem::new(
            "Feed2 Item1",
            "http://feed1.com",
            "",
            "",
            970000000,
            true,
            "",
            1,
        );
        item2.set_ptr(Arc::clone(&f2));
        f2.borrow_mut().add_item(item2);

        let feeds = Vec::from([f1, f2]);
        let result = ctrl.get_query_feeds(&feeds);

        assert!(result.is_ok());
        let qfeeds = result.unwrap();
        assert_eq!(1, qfeeds.len());
        assert_eq!("News", qfeeds[0].title());
        assert_eq!(1, qfeeds[0].items.len());
    }

    #[test]
    fn test_processing_query_feeds_with_multiple_queries() {
        let contents = "
\"query:News:tags # \\\"news\\\" and age < 4 and unread = \\\"yes\\\"\"
            ";
        let reader = UrlReader::init(contents.to_string());
        let ctrl = BuildController {
            url_reader: reader,
            paths: Paths::default(),
            options: Options::default(),
            debug: false,
        };

        let f1 = Arc::new(RefCell::new(Feed::init(
            "".to_string(),
            "Feed1".to_string(),
            "".to_string(),
        )));

        f1.borrow_mut()
            .update_with_url_data(Vec::from(["news".to_string()]), false, None, 0);

        let mut item1 = FeedItem::new(
            "Feed1 Item1",
            "http://feed1.com",
            "",
            "",
            1733974974,
            true,
            "",
            1,
        );
        item1.set_ptr(Arc::clone(&f1));
        f1.borrow_mut().add_item(item1);

        // older than 4 days
        let mut item2 = FeedItem::new(
            "Feed1 Item2",
            "http://feed1.com",
            "",
            "",
            1433974974,
            true,
            "",
            2,
        );
        item2.set_ptr(Arc::clone(&f1));
        f1.borrow_mut().add_item(item2);

        // not unread
        let mut item2 = FeedItem::new(
            "Feed1 Item3",
            "http://feed1.com",
            "",
            "",
            1733974974,
            false,
            "",
            3,
        );
        item2.set_ptr(Arc::clone(&f1));
        f1.borrow_mut().add_item(item2);

        let feeds = Vec::from([f1]);
        let result = ctrl.get_query_feeds(&feeds);
        assert!(result.is_ok());
        let qfeeds = result.unwrap();
        assert_eq!(1, qfeeds.len());
        let feed = qfeeds[0].clone();
        assert_eq!(1, feed.items.len());
        assert_eq!(&"Feed1 Item1".to_string(), feed.items[0].title());
    }

    #[test]
    fn test_populating_url_feeds() {
        let reader = UrlReader::init("".to_string());
        let ctrl = BuildController {
            url_reader: reader,
            paths: Paths::default(),
            options: Options::default(),
            debug: false,
        };

        let f1 = Arc::new(RefCell::new(Feed::init(
            "http://feed1.com".to_string(),
            "Feed1".to_string(),
            "".to_string(),
        )));
        let f2 = Arc::new(RefCell::new(Feed::init(
            "http://feed2.com".to_string(),
            "Feed2".to_string(),
            "".to_string(),
        )));

        let item1 = FeedItem::new(
            "Feed1 Item1",
            "http://feed1.com/1",
            "http://feed1.com",
            "",
            1733974974,
            true,
            "",
            2,
        );

        let item2 = FeedItem::new(
            "Feed1 Item2",
            "http://feed1.com/2",
            "http://feed1.com",
            "",
            1433974974,
            true,
            "",
            2,
        );

        let item3 = FeedItem::new(
            "Feed2 item1",
            "http://feed2.com/1",
            "http://feed2.com",
            "",
            1733974974,
            false,
            "",
            3,
        );
        let item4 = FeedItem::new(
            "Feed0 item1 ",
            "http://feed0.com/1",
            "http://feed0.com",
            "",
            1733974974,
            false,
            "",
            4,
        );
        let feeds = Vec::from([f1.clone(), f2.clone()]);
        let items = Vec::from([item1, item2, item3, item4]);

        ctrl.populate_url_feeds(&feeds, &items);
        assert_eq!(2, f1.borrow().items.len());
        assert_eq!(1, f2.borrow().items.len());
    }

    #[test]
    fn test_populating_url_feeds_with_unread_filtering() {
        let reader = UrlReader::init("".to_string());
        let mut opts = Options::default();
        opts.show_read_articles = false;
        let ctrl = BuildController {
            url_reader: reader,
            paths: Paths::default(),
            options: opts,
            debug: false,
        };

        let f1 = Arc::new(RefCell::new(Feed::init(
            "http://feed1.com".to_string(),
            "Feed1".to_string(),
            "".to_string(),
        )));

        let item1 = FeedItem::new(
            "Feed1 Item1",
            "http://feed1.com/1",
            "http://feed1.com",
            "",
            1733974974,
            true,
            "",
            2,
        );

        let item2 = FeedItem::new(
            "Feed1 Item2",
            "http://feed1.com/2",
            "http://feed1.com",
            "",
            1433974974,
            false,
            "",
            2,
        );

        let feeds = Vec::from([f1.clone()]);
        let items = Vec::from([item1, item2]);

        ctrl.populate_url_feeds(&feeds, &items);
        assert_eq!(1, f1.borrow().items.len());
    }

    #[test]
    fn test_processing_url_feeds() {
        use crate::db::MockConnector;

        let contents = "
http://feed1.com \"~Some feed\" dev
http://feed2.com \"~Some feed 2\"
http://feed3.com
            ";
        let reader = UrlReader::init(contents.to_string());
        let ctrl = BuildController {
            url_reader: reader,
            paths: Paths::default(),
            options: Options::default(),
            debug: false,
        };

        let mut db_mock = MockConnector::new();
        let f1 = Feed::init(
            "http://feed1.com".to_string(),
            "Feed1".to_string(),
            "".to_string(),
        );
        let f2 = Feed::init(
            "http://feed2.com".to_string(),
            "Feed2".to_string(),
            "".to_string(),
        );
        let f3 = Feed::init(
            "http://feed3.com".to_string(),
            "Feed3".to_string(),
            "".to_string(),
        );
        db_mock
            .expect_get_feeds()
            .return_once_st(move |_| Ok(Vec::from([f1.clone(), f2.clone(), f3.clone()].clone())));
        let results = ctrl.get_url_feeds(&db_mock);
        assert!(results.is_ok());
        let feeds = results.unwrap();
        assert_eq!(3, feeds.len());
        assert_eq!("Feed1", feeds[0].borrow().title());
        assert_eq!("Some feed", feeds[0].borrow().display_title());
        assert_eq!("Feed2", feeds[1].borrow().title());
        assert_eq!("Some feed 2", feeds[1].borrow().display_title());
        assert_eq!("Feed3", feeds[2].borrow().title());
    }
}
