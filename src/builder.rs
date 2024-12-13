use log::info;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

use handlebars::Handlebars;

use crate::feed::{Feed, FeedList};
use crate::template::Context;

const FEEDS_DIRNAME: &str = "feeds";
const INCLUDE_DIRNAME: &str = "include";
const INDEX_FILENAME: &str = "index";
const BUILD_TIME_FILENAME: &str = "build_time.txt";

/// This trait defines methods which every builder representation
/// must implement.
pub trait Builder {
    fn create_tmp(&self) -> Result<(), io::Error>;
    fn generate_aux_data(&self) -> Result<(), Box<dyn Error>>;
    fn render_templates(&self) -> Result<(), Box<dyn Error>>;
    fn copy_data(&self) -> Result<(), Box<dyn Error>>;
    fn clean_up(&self);
}

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
        Ok(())
    }

    /// Generate any auxiliary data required for page generation,
    /// such as json feeds.
    fn generate_aux_data(&self) -> Result<(), Box<dyn Error>> {
        self.save_json_feeds()?;
        self.save_build_time()?;
        Ok(())
    }

    /// Copy data from tmp to build directory.
    fn copy_data(&self) -> Result<(), Box<dyn Error>> {
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

        let tpl_index_path = self.tmp_dir.join(format!("{}.html", INDEX_FILENAME));
        let index_path = self.build_dir.join(format!("{}.html", INDEX_FILENAME));
        info!(
            "Copying rendered index @ {} to {}",
            tpl_index_path.display(),
            index_path.display()
        );
        fs::copy(tpl_index_path, index_path)?;
        let b_time_t_path = self.tmp_dir.join(BUILD_TIME_FILENAME);
        let b_time_path = self.build_dir.join(BUILD_TIME_FILENAME);
        fs::copy(b_time_t_path, b_time_path)?;
        Ok(())
    }

    /// Render template using context provided.
    fn render_templates(&self) -> Result<(), Box<dyn Error>> {
        let tpl_file = self.template_path.join(format!("{}.hbs", INDEX_FILENAME));
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
    fn save_build_time(&self) -> Result<(), Box<dyn Error>> {
        let path = self.tmp_dir.join(BUILD_TIME_FILENAME);
        let mut file = File::create(path)?;
        file.write_all(format!("{}", self.context.build_time()).as_bytes())?;
        Ok(())

    }

    /// Save single feed data in tmp dir.
    fn save_feed_data(&self, name: &String, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let feeds_dir = self.tmp_dir.join(FEEDS_DIRNAME);
        let path = feeds_dir.join(format!("{}.json", name));
        info!("Saving feed at path {}", path.display());
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }

    /// Generate json files for each feed.
    fn save_json_feeds(&self) -> Result<(), Box<dyn Error>> {
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
    fn save_json_feedlist(&self, feedlist: &FeedList, name: String) -> Result<(), Box<dyn Error>> {
        if self.debug {
            self.save_feed_data(&name, serde_json::to_string_pretty(&feedlist)?.as_bytes())?;
        } else {
            self.save_feed_data(&name, serde_json::to_string(&feedlist)?.as_bytes())?;
        }
        Ok(())
    }

    /// Save single feed items.
    fn save_json_feed(&self, feed: &Feed) -> Result<(), Box<dyn Error>> {
        if feed.is_empty() || feed.is_hidden() {
            info!("Skipping saving feed: {:?}", feed);
            return Ok(());
        }
        //TODO: add archives
        if self.debug {
            self.save_feed_data(feed.id(), serde_json::to_string_pretty(&feed)?.as_bytes())?;
        } else {
            self.save_feed_data(feed.id(), serde_json::to_string(&feed)?.as_bytes())?;
        }
        Ok(())
    }
}

/// Helper func for copying all the contents of directory
/// to another.
fn copy_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
