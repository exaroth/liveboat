// TODO use current dir by default for building
// if unnamed arg passed use that instead

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::Path;
use std::{fs, io};

use handlebars::Handlebars;

const FEEDS_DIRNAME: &str = "feeds";
const INCLUDE_DIRNAME: &str = "include";
const INDEX_FILENAME: &str = "index";

pub struct SinglePageBuilder<'a, C> {
    template_path: &'a Path,
    build_dir: &'a Path,
    tmp_dir: &'a Path,
    context: C,
}

impl<'a, C: serde::Serialize> SinglePageBuilder<'a, C> {
    pub fn init(
        tmp_dir: &'a Path,
        build_dir: &'a Path,
        template_path: &'a Path,
        template_name: &String,
        context: C,
    ) -> Result<SinglePageBuilder<'a, C>, IOError> {
        // TODO: use src when dev mode is enabled
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
        // TODO: remove after
        let tpl_file = template_path.join(format!("{}.hbs", INDEX_FILENAME));
        if !tpl_file.try_exists()? {
            return Err(IOError::new(
                ErrorKind::NotFound,
                format!("index.hpl does not exist for template {}", template_name),
            )
            .into());
        };

        Ok(SinglePageBuilder {
            template_path,
            build_dir,
            tmp_dir,
            context,
        })
    }

    pub fn create_tmp(&self) {
        _ = fs::create_dir(self.tmp_dir);
        _ = fs::create_dir(self.tmp_dir.join(FEEDS_DIRNAME));
    }

    pub fn save_feed_data(&self, name: &String, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let feeds_dir = self.tmp_dir.join(FEEDS_DIRNAME);
        let path = feeds_dir.join(format!("{}.json", name));
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }

    pub fn copy_data(&self) -> Result<(), Box<dyn Error>> {
        let feeds_dir_tmp = self.tmp_dir.join(FEEDS_DIRNAME);
        let feeds_dir = self.build_dir.join(FEEDS_DIRNAME);

        if self.build_dir.is_dir() && feeds_dir.is_dir() {
            _ = fs::remove_dir_all(&feeds_dir);
        }
        copy_all(feeds_dir_tmp, &feeds_dir)?;

        let include_dir = self.template_path.join(INCLUDE_DIRNAME);
        copy_all(include_dir, &self.build_dir)?;

        let tpl_index_path = self.tmp_dir.join(format!("{}.html", INDEX_FILENAME));
        let index_path = self.build_dir.join(format!("{}.html", INDEX_FILENAME));
        fs::copy(tpl_index_path, index_path)?;
        Ok(())
    }

    pub fn render_template(&self) -> Result<(), Box<dyn Error>> {
        let tpl_file = self.template_path.join(format!("{}.hbs", INDEX_FILENAME));
        let raw = fs::read_to_string(tpl_file)?;
        let mut handlebars = Handlebars::new();
        _ = handlebars.register_template_string(INDEX_FILENAME, raw);
        let out = handlebars.render(INDEX_FILENAME, &self.context)?;

        let out_path = self.tmp_dir.join(format!("{}.html", INDEX_FILENAME));
        let mut f = File::create(out_path)?;
        f.write_all(out.as_bytes())?;

        Ok(())
    }

    pub fn clean_up(&self) {
        _ = fs::remove_dir_all(self.tmp_dir);
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
