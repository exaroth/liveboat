use thiserror::Error;

#[derive(Error, Debug)]
pub enum FilesystemError {
    #[error("Invalid path provided: `{0}`")]
    InvalidPathProvided(String),
    #[error("File or directory does not exist: `{0}`")]
    PathDoesNotExist(String),
    #[error("Looks like liveboat has not been initialized, run liveboat -x init first.")]
    NotInitialized,

    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("Unknown filesystem error `{0}`")]
    Unknown(String),
}
