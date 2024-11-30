use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::str;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Options {
    title: String,
    remote_url: String,
    show_read_articles: bool,
    template_name: String,

    // TODO
    time_threshold: u8,
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
            show_read_articles: false,
            template_name: String::from("default"),
            time_threshold: 10,
            exclude_tags: Vec::new(),
            exclude_feeds: Vec::new(),
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
