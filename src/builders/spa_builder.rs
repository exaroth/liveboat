use log::info;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

use anyhow::Result;
use handlebars::Handlebars;
use url::Url;

use crate::builders::aux::Builder;
use crate::builders::utils::{generate_opml, generate_rss_channel};
use crate::errors::ConfigurationError;
use crate::feed::{Feed, FeedList};
use crate::template::Context;
use crate::utils::copy_all;

/// Dirname of the directory to be used for storing feed data.
const FEEDS_DIRNAME: &str = "feeds";
/// Dirname of include directory.
const INCLUDE_DIRNAME: &str = "include";
/// Base name of the index template and output html page.
const INDEX_FILENAME: &str = "index";
/// Filename for the file containing build timestamp.
const BUILD_TIME_FILENAME: &str = "build_time.txt";
/// Filename of the rss file.
const RSS_FILE_FILENAME: &str = "rss.xml";
/// Directory name used for storing self referential rss documents
/// used for storing query feed rss data.
const SELF_REFERENTIAL_RSS_DIRNAME: &str = "channels";
/// Filename of the ompl file.
const OPML_FILENAME: &str = "opml.xml";

/// This represents default builder module
/// used for processing single page Liveboat templates.
pub struct SinglePageBuilder<'a, C>
where
    C: serde::Serialize + Context,
{
    template_path: &'a Path,
    build_dir: &'a Path,
    tmp_dir: &'a Path,
    context: &'a C,
    debug: bool,
}

impl<'a, C> Builder for SinglePageBuilder<'a, C>
where
    C: serde::Serialize + Context,
{
    /// Create tmp directory structure.
    fn create_tmp(&self) -> Result<(), io::Error> {
        info!("Creating tmp dir at {}", self.tmp_dir.display());
        _ = fs::create_dir(self.tmp_dir)?;
        info!("Creating tmp feeds dir");
        _ = fs::create_dir(self.tmp_dir.join(FEEDS_DIRNAME))?;
        _ = fs::create_dir(self.tmp_dir.join(SELF_REFERENTIAL_RSS_DIRNAME))?;
        Ok(())
    }

    /// Generate any auxiliary data required for page generation,
    /// such as json feeds.
    fn generate_aux_data(&self) -> Result<()> {
        self.save_json_feeds()?;
        self.save_build_time()?;
        self.save_rss_channel()?;
        self.save_query_feed_channels()?;
        self.save_opml()?;
        Ok(())
    }

    /// Copy data from tmp to build directory.
    fn copy_data(&self) -> Result<()> {
        let include_dir = self.template_path.join(INCLUDE_DIRNAME);
        info!("Copying include contents @ {}", include_dir.display());
        copy_all(include_dir, &self.build_dir)?;

        let feeds_dir_tmp = self.tmp_dir.join(FEEDS_DIRNAME);
        let feeds_dir = self.build_dir.join(FEEDS_DIRNAME);
        info!(
            "Copying feed data from {} to {}",
            feeds_dir_tmp.display(),
            feeds_dir.display()
        );

        if self.build_dir.is_dir() && feeds_dir.is_dir() {
            _ = fs::remove_dir_all(&feeds_dir);
        }
        copy_all(feeds_dir_tmp, &feeds_dir)?;

        let channel_dir_tmp = self.tmp_dir.join(SELF_REFERENTIAL_RSS_DIRNAME);
        let channel_dir = self.build_dir.join(SELF_REFERENTIAL_RSS_DIRNAME);
        copy_all(channel_dir_tmp, &channel_dir)?;

        let tpl_index_path =
            self.tmp_dir.join(format!("{}.html", INDEX_FILENAME));
        let index_path =
            self.build_dir.join(format!("{}.html", INDEX_FILENAME));
        info!(
            "Copying rendered index @ {} to {}",
            tpl_index_path.display(),
            index_path.display()
        );
        fs::copy(tpl_index_path, index_path)?;
        fs::copy(
            self.tmp_dir.join(BUILD_TIME_FILENAME),
            self.build_dir.join(BUILD_TIME_FILENAME),
        )?;
        fs::copy(
            self.tmp_dir.join(RSS_FILE_FILENAME),
            self.build_dir.join(RSS_FILE_FILENAME),
        )?;
        fs::copy(
            self.tmp_dir.join(OPML_FILENAME),
            self.build_dir.join(OPML_FILENAME),
        )?;
        Ok(())
    }

    /// Render template using context provided.
    fn render_templates(&self) -> Result<()> {
        let tpl_file =
            self.template_path.join(format!("{}.hbs", INDEX_FILENAME));
        info!("Rendering template @ {}", &tpl_file.display());
        let raw = fs::read_to_string(tpl_file)?;
        let mut handlebars = Handlebars::new();
        _ = handlebars.register_template_string(INDEX_FILENAME, raw);
        let out = handlebars.render(INDEX_FILENAME, &self.context)?;

        let out_path = self.tmp_dir.join(format!("{}.html", INDEX_FILENAME));
        info!("Saving template @ {}", &out_path.display());

        let mut f = File::create(out_path)?;
        f.write_all(out.as_bytes())?;

        Ok(())
    }

    /// Clean up tmp directory.
    fn clean_up(&self) {
        info!("Cleanup");
        _ = fs::remove_dir_all(self.tmp_dir);
    }
}

impl<'a, C> SinglePageBuilder<'a, C>
where
    C: serde::Serialize + Context,
{
    pub fn init(
        tmp_dir: &'a Path,
        build_dir: &'a Path,
        template_path: &'a Path,
        context: &'a C,
        debug: bool,
    ) -> Result<SinglePageBuilder<'a, C>, IOError> {
        if !template_path.try_exists()? {
            return Err(IOError::new(
                ErrorKind::NotFound,
                format!(
                    "Template not found at path {}",
                    template_path.to_str().unwrap()
                ),
            )
            .into());
        }
        let tpl_file = template_path.join(format!("{}.hbs", INDEX_FILENAME));
        if !tpl_file.try_exists()? {
            return Err(IOError::new(
                ErrorKind::NotFound,
                format!(
                    "index.hpl does not exist for path {}",
                    template_path.display()
                ),
            )
            .into());
        };

        Ok(SinglePageBuilder {
            template_path,
            build_dir,
            tmp_dir,
            context,
            debug,
        })
    }

    /// Save build time as text file.
    fn save_build_time(&self) -> Result<()> {
        let path = self.tmp_dir.join(BUILD_TIME_FILENAME);
        let mut file = File::create(path)?;
        file.write_all(format!("{}", self.context.build_time()).as_bytes())?;
        Ok(())
    }

    /// Save atom feed for all the feeds
    fn save_rss_channel(&self) -> Result<()> {
        let path = self.tmp_dir.join(RSS_FILE_FILENAME);
        let mut file = File::create(path)?;
        file.write_all(
            generate_rss_channel(
                self.context.options(),
                self.context.feeds(),
                true,
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    /// Save atom feed for all the feeds
    fn save_opml(&self) -> Result<()> {
        let url = Url::parse(self.context.options().site_url.as_str());
        if url.is_err() {
            return Err(ConfigurationError::InvalidSiteUrl.into());
        }
        let path = self.tmp_dir.join(OPML_FILENAME);
        let mut file = File::create(path)?;
        file.write_all(
            generate_opml(
                self.context.options(),
                self.context.feeds(),
                &url.unwrap(),
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    /// Save single feed data in tmp dir.
    fn save_feed_data(&self, name: &String, data: &[u8]) -> Result<()> {
        let feeds_dir = self.tmp_dir.join(FEEDS_DIRNAME);
        let path = feeds_dir.join(format!("{}.json", name));
        info!("Saving feed at path {}", path.display());
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }

    /// Save rss feeds containing query feed data used in the
    /// self referential data used in OPML channel.
    fn save_query_feed_channels(&self) -> Result<()> {
        let base_path = self.tmp_dir.join(SELF_REFERENTIAL_RSS_DIRNAME);
        let q_feeds: Vec<&Feed> = self
            .context
            .feeds()
            .into_iter()
            .filter(|f| f.is_query_feed() == true)
            .collect();
        for f in q_feeds {
            let path = base_path.join(format!("{}.xml", f.id()));
            info!("Saving channel data for query feed: {}", path.display());
            let mut file = File::create(path)?;
            file.write_all(
                generate_rss_channel(
                    self.context.options(),
                    &Vec::from([f.clone()]),
                    false,
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }

    /// Generate json files for each feed.
    fn save_json_feeds(&self) -> Result<()> {
        let mut f_list = FeedList::new();
        for f in self.context.feeds() {
            if !f.is_empty() && !f.is_hidden() {
                f_list.add_feed(&f);
                self.save_json_feed(f)?;
            }
        }
        self.save_json_feedlist(&f_list, String::from("feeds"))?;
        Ok(())
    }

    /// Save list of all feeds.
    fn save_json_feedlist(
        &self,
        feedlist: &FeedList,
        name: String,
    ) -> Result<()> {
        if self.debug {
            self.save_feed_data(
                &name,
                serde_json::to_string_pretty(&feedlist)?.as_bytes(),
            )?;
        } else {
            self.save_feed_data(
                &name,
                serde_json::to_string(&feedlist)?.as_bytes(),
            )?;
        }
        Ok(())
    }

    /// Save single feed items.
    fn save_json_feed(&self, feed: &Feed) -> Result<()> {
        if feed.is_empty() || feed.is_hidden() {
            info!("Skipping saving feed: {:?}", feed);
            return Ok(());
        }
        let mut truncated = feed.clone();
        truncated.truncate_items();
        if self.debug {
            self.save_feed_data(
                truncated.id(),
                serde_json::to_string_pretty(&truncated)?.as_bytes(),
            )?;
            self.save_feed_data(
                &format!("{}_archive", feed.id()),
                serde_json::to_string_pretty(&feed)?.as_bytes(),
            )?;
        } else {
            self.save_feed_data(
                truncated.id(),
                serde_json::to_string(&truncated)?.as_bytes(),
            )?;
            self.save_feed_data(
                &format!("{}_archive", feed.id()),
                serde_json::to_string(&feed)?.as_bytes(),
            )?;
        }
        Ok(())
    }
}
