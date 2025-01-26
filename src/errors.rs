use std::path::PathBuf;
use thiserror::Error;

/// Error used for handling common issues related
/// to filesystem such as missing files.
#[derive(Error, Debug)]
pub enum FilesystemError {
    // #[error("Invalid path provided: `{0}`")]
    // InvalidPathProvided(String),
    #[error("File or directory does not exist: `{0}`")]
    PathDoesNotExist(PathBuf),
    #[error("Looks like liveboat has not been initialized, run liveboat -x init first.")]
    NotInitialized,

    #[error("Unknown filesystem error `{0}`")]
    Unknown(String),
}

/// Errors returned during url reader processing
#[derive(Error, Debug)]
pub enum UrlReaderError {
    #[error("Invalid query found: {0}")]
    InvalidQueryError(String),

    #[error("Error matching query feed {0}")]
    MatcherError(String),
}

/// Errors related to invalid configuration settings passed
/// by the user.
#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Invalid site URL defined in Liveboat options")]
    InvalidSiteUrl,
}
