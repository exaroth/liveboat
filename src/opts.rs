use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use std::str;

use anyhow::Result;
use toml;

const fn default_bool<const V: bool>() -> bool {
    V
}
const fn default_u64<const V: u64>() -> u64 {
    V
}

fn default_title() -> String {
    String::from("Liveboat feed page")
}

fn default_site_path() -> String {
    String::from("/")
}

fn default_site_address() -> String {
    String::from("https://example.com")
}

fn default_newsboat_url_file() -> String {
    String::from("")
}

fn default_newsboat_cache_file() -> String {
    String::from("")
}

fn default_build_dir() -> String {
    String::from("")
}

fn default_template_name() -> String {
    String::from("default")
}

/// This represents main configuration options
/// available to the user.
#[derive(Deserialize, Serialize, Debug)]
pub struct Options {
    /// Title of the page
    #[serde(default = "default_title")]
    pub title: String,
    /// Root path for the feed site
    #[serde(default = "default_site_path")]
    pub site_path: String,
    /// Optional address to your website, for
    /// github pages it will be https://<username>.github.io
    #[serde(default = "default_site_address")]
    pub site_address: String,
    /// Whether or not to show articles marked as read by Newsboat
    #[serde(default = "default_bool::<true>")]
    pub show_read_articles: bool,
    /// Define whether or not to include article content in generated
    /// rss feeds (might increase size significantly)
    #[serde(default = "default_bool::<false>")]
    pub include_article_content_in_rss_feeds: bool,
    /// Path to Newsboat urls file
    #[serde(default = "default_newsboat_url_file")]
    pub newsboat_urls_file: String,
    /// Path to Newsboat cache file
    #[serde(default = "default_newsboat_cache_file")]
    pub newsboat_cache_file: String,
    /// Number of days in the past for which to process articles for
    #[serde(default = "default_u64::<20>")]
    pub time_threshold: u64,
    /// Path to directory containing output files
    #[serde(default = "default_build_dir")]
    pub build_dir: String,
    /// Name of the template to use
    #[serde(default = "default_template_name")]
    pub template_name: String,
}

impl Options {
    pub fn init(path: &Path) -> Result<Options> {
        if !path.exists() {
            return Ok(Options::default());
        };
        let result = Options::load(path)?;
        Ok(result)
    }

    /// Initialize default option settings.
    pub fn default() -> Options {
        return Options {
            title: default_title(),
            site_path: default_site_path(),
            site_address: default_site_address(),
            show_read_articles: true,
            template_name: default_template_name(),
            include_article_content_in_rss_feeds: false,
            time_threshold: 20,
            newsboat_urls_file: default_newsboat_url_file(),
            newsboat_cache_file: default_newsboat_cache_file(),
            build_dir: default_build_dir(),
        };
    }

    /// Output data to TOML and save.
    pub fn save(&self, path: &Path) -> Result<String> {
        let t = toml::to_string(&self)?;

        let mut f = File::create(path)?;
        f.write_all(t.as_bytes())?;
        Ok(t)
    }

    /// Instantiate options from TOML file.
    pub fn load(path: &Path) -> Result<Options> {
        let raw = read_to_string(path)?;
        let opts: Options = toml::from_str(raw.as_str())?;
        return Ok(opts);
    }

    pub fn template_name(&self) -> &String {
        return &self.template_name;
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Opts::
            title {}:
            site_path {}:
            show_read: {}
            template_name: {}
            urls_file: {}
            cache_file: {}
            time_threshold: {},
            build_dir: {}",
            self.title,
            self.site_path,
            self.show_read_articles,
            self.template_name,
            self.newsboat_urls_file,
            self.newsboat_cache_file,
            self.time_threshold,
            self.build_dir,
        )
    }
}
