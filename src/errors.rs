use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum FilesystemError {
    #[error("Invalid path provided: `{0}`")]
    InvalidPathProvided(String),
    #[error("File or directory does not exist: `{0}`")]
    PathDoesNotExist(PathBuf),
    #[error("Looks like liveboat has not been initialized, run liveboat -x init first.")]
    NotInitialized,

    #[error("Unknown filesystem error `{0}`")]
    Unknown(String),
}
