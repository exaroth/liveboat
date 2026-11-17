use anyhow::Result;

/// Dirname of the directory to be used for storing feed data.
pub const FEEDS_DIRNAME: &str = "feeds";
/// Dirname of include directory.
pub const INCLUDE_DIRNAME: &str = "include";
/// Base name of the index template and output html page.
pub const INDEX_TEMPLATE_FILENAME: &str = "index";
/// Base name of the single feed template
pub const FEED_TEMPLATE_FILENAME: &str = "feed";
/// Base name of the single article template
pub const ARTICLE_TEMPLATE_FILENAME: &str = "article";
/// Filename for the file containing build timestamp.
pub const BUILD_TIME_FILENAME: &str = "build_time.txt";
/// Filename of the rss file.
pub const RSS_FILE_FILENAME: &str = "rss.xml";
/// Directory name used for storing self referential rss documents
/// used for storing query feed rss data.
pub const SELF_REFERENTIAL_RSS_DIRNAME: &str = "channels";
/// Filename of the ompl file.
pub const OPML_FILENAME: &str = "opml.xml";

/// This trait defines methods which every builder representation
/// must implement.
pub trait Builder {
    fn render_templates(&self) -> Result<()>;
    fn generate_aux_data(&self) -> Result<()>;
}
