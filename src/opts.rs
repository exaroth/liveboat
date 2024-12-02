use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::str;
use toml;
use std::fmt;

#[derive(Deserialize, Serialize, Debug)]
pub struct Options {
    pub title: String,
    remote_url: String,
    pub show_read_articles: bool,
    template_name: String,

    // TODO
    pub newsboat_urls_file: String,
    pub newsboat_cache_file: String,
    pub time_threshold: u64,
    pub build_dir: String,
    exclude_tags: Vec<String>,
    exclude_feeds: Vec<String>,
    // TODO
}

impl Options {

    pub fn init(path: &Path) -> Result<Options, Box<dyn std::error::Error>> {
        if !path.exists() {
            return Ok(Options::default());
        };
        let result = Options::load(path)?;
        Ok(result)
    }

    /// Initialize default option settings.
    pub fn default() -> Options {
        return Options {
            title: String::from("Liveboat feed page"),
            remote_url: String::new(),
            show_read_articles: true,
            template_name: String::from("default"),
            time_threshold: 20,
            exclude_tags: Vec::new(),
            exclude_feeds: Vec::new(),
            newsboat_urls_file: String::new(),
            newsboat_cache_file: String::new(),
            build_dir: String::new(),
        };
    }

    pub fn save(&self, path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let t = toml::to_string(&self)?;

        if !path.exists() {
            File::create(path)?;
        }
        let mut f = File::create(path)?;
        f.write_all(t.as_bytes())?;
        Ok(t)
    }

    pub fn load(path: &Path) -> Result<Options, Box<dyn std::error::Error>> {
        let raw = read_to_string(path)?;
        let opts: Options = toml::from_str(raw.as_str())?;
        return Ok(opts);
    }

    pub fn template_name(&self) -> &String {
        return &self.template_name
    }

}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Paths::
            title {}:
            show_read: {}
            template_name: {}
            urls_file: {}
            cache_file: {}
            time_threshold: {},
            build_dir: {}",
            self.title,
            self.show_read_articles,
            self.template_name,
            self.newsboat_urls_file,
            self.newsboat_cache_file,
            self.time_threshold,
            self.build_dir,
        )
    }
}
