/// This module contains logic related to processing Newsboat urls files.
use log::info;
use std::fmt;

use anyhow::{Error, Result};

use libnewsboat::matcher::Matcher;
use libnewsboat::utils as libutils;

use crate::errors::UrlReaderError;

/// Representation of single url based feed,
/// this will also include file based feeds.
#[derive(Debug, Clone)]
pub struct URLFeed {
    pub url: String,
    pub tags: Vec<String>,
    pub hidden: bool,
    pub title_override: Option<String>,
    pub line_no: usize,
}

/// Representation of query based feed.
pub struct QueryFeed {
    pub title: String,
    pub line_no: usize,
    pub matcher: Matcher,
}

impl fmt::Display for QueryFeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Paths::
            title {}:
            line_no: {}",
            self.title, self.line_no,
        )
    }
}

/// Module used for operating on the
/// Newsboat urls file.
#[derive(Clone, Debug)]
pub struct UrlReader {
    lines: Vec<String>,
}

impl UrlReader {
    pub fn init(url_file: String) -> UrlReader {
        info!("Initializing reader");
        let mut u = UrlReader { lines: Vec::new() };
        u.read(url_file);
        return u;
    }

    /// Read the lines of the urls file, filtering out
    /// comments and empty lines.
    fn read(&mut self, url_file: String) {
        let mut result: Vec<String> = Vec::new();
        for line in url_file.lines() {
            info!("Reading line {}", line);
            let l = line.trim();
            if l.is_empty() || l.starts_with("#") {
                info!("Ignoring");
                continue;
            }
            result.push(String::from(line));
        }
        self.lines = result;
    }

    /// Process tokens associated with single url feed.
    fn get_http_feed(&self, tokens: &Vec<String>, line_no: usize) -> URLFeed {
        info!("Processing http feed");
        let mut feed = URLFeed {
            url: String::from(&tokens[0]),
            tags: Vec::new(),
            hidden: false,
            title_override: None,
            line_no: line_no,
        };
        info!("Default feed: {}", format!("{:?}", feed));
        let l = tokens.len();
        if l > 1 {
            for i in 1..l {
                info!("Processing token: {}", tokens[i]);
                if tokens[i].starts_with("~") {
                    info!("Title token found");
                    let mut t_override = tokens[i].clone();
                    t_override.remove(0);
                    feed.title_override = Some(t_override);
                    continue;
                }
                if tokens[i] == "!" {
                    info!("Setting feed to hidden");
                    feed.hidden = true;
                    continue;
                }
                info!("Adding tag");
                feed.tags.push(String::from(&tokens[i]))
            }
        }
        info!("URL feed after: {}", format!("{:?}", feed));
        return feed;
    }

    /// Fetch all url feeds as defined in urls file.
    pub fn get_url_feeds(&self) -> Vec<URLFeed> {
        info!("Retrieving url feeds from url file");
        let mut result = Vec::new();
        let linel = self.lines.len();
        for line_no in 0..linel {
            let line = &self.lines[line_no];
            info!("Line: {}", line);
            let tokens = libutils::tokenize_quoted(line.as_str(), " \r\n\t");
            info!("Tokens: {}", format!("{:?}", tokens));
            if tokens.is_empty() {
                info!("Is empty, Skipping");
                continue;
            }
            if !libutils::is_http_url(tokens[0].as_str()) && !tokens[0].starts_with("file://") {
                info!("Is special, skipping");
                continue;
            }
            let feed = self.get_http_feed(&tokens, line_no);
            result.push(feed);
        }
        result
    }

    /// Fetch all query urls as defined in urls file.
    pub fn get_query_urls(&self) -> Result<Vec<QueryFeed>, UrlReaderError> {
        info!("Retrieving query urls");
        let mut results = Vec::new();
        let linel = self.lines.len();
        for line_no in 0..linel {
            let line = &self.lines[line_no];
            let tokens = libutils::tokenize_quoted(line.as_str(), " \r\n\t");
            if tokens.is_empty() {
                continue;
            }
            if !libutils::is_query_url(tokens[0].as_str()) {
                info!("Skipping line: {}", format!("{:?}", tokens));
                continue;
            }
            let parts = libutils::tokenize_quoted(tokens[0].as_str(), ":");
            info!("Parts are: {}", format!("{:?}", parts));
            if parts.len() < 3 {
                return Err(UrlReaderError::InvalidQueryError(format!(
                    "Invalid query found: {}",
                    line
                )));
            }
            let filter_s = &parts[2];
            info!("Matching against: {}", filter_s);
            match Matcher::parse(filter_s) {
                Ok(r) => results.push(QueryFeed {
                    title: parts[1].clone(),
                    matcher: r,
                    line_no: line_no,
                }),
                Err(e) => return Err(UrlReaderError::MatcherError(e)),
            };
        }
        Ok(results)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_reading_empty_or_commented_lines() {
        let contents = "
# Test

            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 0);
    }

    #[test]
    fn test_reading_url_or_query_lines() {
        let contents = "
https://hnrss.org/best \"~HN\" dev
exec:~/.scripts/pocket_atom
file:///home/exaroth/.scripts/atom.xml \"~ Atom Pocket\"
\"query:Youtube:tags # \\\"yt\\\"\"
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 4);
    }
    #[test]
    fn test_processing_url_feeds_simple() {
        let contents = "
https://hnrss.org/best
http://aphyr.com/posts.atom
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 2);
        let feeds = reader.get_url_feeds();
        assert_eq!(2, feeds.len());
        assert_eq!(0, feeds[0].line_no);
        assert_eq!("https://hnrss.org/best", feeds[0].url);
        assert_eq!(false, feeds[0].hidden);
        assert_eq!(0, feeds[0].tags.len());
        assert_eq!(None, feeds[0].title_override);
        assert_eq!(1, feeds[1].line_no);
        assert_eq!("http://aphyr.com/posts.atom", feeds[1].url);
        assert_eq!(false, feeds[1].hidden);
        assert_eq!(None, feeds[1].title_override);
        assert_eq!(0, feeds[1].tags.len());
    }
    #[test]
    fn test_processing_url_feeds_with_title_override() {
        let contents = "
https://hnrss.org/best \"~Override\"
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 1);
        let feeds = reader.get_url_feeds();
        assert_eq!(1, feeds.len());
        assert!(feeds[0].title_override.is_some());
        assert_eq!(Some(String::from("Override")), feeds[0].title_override);
    }
    #[test]
    fn test_processing_url_feeds_with_title_with_tags() {
        let contents = "
https://hnrss.org/best \"~Override\" dev news
https://techcrunch.com/feed/ tech
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 2);
        let feeds = reader.get_url_feeds();
        assert_eq!(2, feeds.len());
        assert_eq!(2, feeds[0].tags.len());
        assert_eq!(Vec::from(["dev", "news"]), feeds[0].tags);
        assert_eq!(1, feeds[1].tags.len());
        assert_eq!(Vec::from(["tech"]), feeds[1].tags);
    }
    #[test]
    fn test_processing_url_feeds_hidden() {
        let contents = "
https://hnrss.org/best \"~Override\" ! dev news
https://techcrunch.com/feed/ ! tech
https://kubernetes.io/feed.xml \"~Dev - Kubernetes Blog\" !
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 3);
        let feeds = reader.get_url_feeds();
        assert_eq!(3, feeds.len());
        assert_eq!(true, feeds[0].hidden);
        assert_eq!(true, feeds[1].hidden);
        assert_eq!(true, feeds[2].hidden);
    }
    #[test]
    fn test_processing_invalid_url_feeds() {
        let contents = "
git+https://invalid.com
gibberish
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 2);
        let feeds = reader.get_url_feeds();
        assert_eq!(0, feeds.len());
    }
    #[test]
    fn test_processing_file_based_feeds() {
        let contents = "
file:///home/exaroth/.scripts/atom.xml \"~Atom Pocket\"
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 1);
        let feeds = reader.get_url_feeds();
        assert_eq!(1, feeds.len());
        assert_eq!(
            "file:///home/exaroth/.scripts/atom.xml".to_string(),
            feeds[0].url
        );
        assert_eq!(Some("Atom Pocket".to_string()), feeds[0].title_override);
    }
    #[test]
    fn test_processing_exec_statements() {
        let contents = "
exec:~/.scripts/pocket_atom
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 1);
        let feeds = reader.get_url_feeds();
        assert_eq!(0, feeds.len());
    }
    #[test]
    fn test_processing_query_urls_simple() {
        let contents = "
\"query:Youtube:tags # \\\"yt\\\"\"
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 1);
        let result = reader.get_query_urls();
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(1, data.len());
        assert_eq!("Youtube".to_string(), data[0].title);
        assert_eq!(0, data[0].line_no);
    }
    #[test]
    fn test_processing_query_urls_with_multiple_filters() {
        let contents = "
\"query:News:tags # \\\"news\\\" and age < 1 and unread = \\\"yes\\\"\"
            ";
        let reader = UrlReader::init(contents.to_string());
        assert_eq!(reader.lines.len(), 1);
        let result = reader.get_query_urls();
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(1, data.len());
        assert_eq!("News".to_string(), data[0].title);
        assert_eq!(0, data[0].line_no);
    }
    #[test]
    fn test_processing_query_urls_invalid() {
        let contents = "
\"query:Gibberish\"
            ";
        let result = UrlReader::init(contents.to_string()).get_query_urls();
        assert!(result.is_err());
        let contents = "
\"query:Youtube:tags # yt\"
            ";
        let result = UrlReader::init(contents.to_string()).get_query_urls();
        assert!(result.is_err());
    }
}
