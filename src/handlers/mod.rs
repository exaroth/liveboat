/// This module contains implementation of command handlers
/// such as building pages, initializing configuration or updating
/// files.
pub mod build;
pub mod init;
pub mod update;

mod aux;

/// Base path to Github releae to fetch binaries/templates from
pub const RELEASE_CHANNEL: &str =
    "https://github.com/exaroth/liveboat/releases/download";
/// Tag name for release channel
pub const STABLE_CHANNEL_NAME: &str = "stable";
/// Tag name for development channel
pub const NIGHTLY_CHANNEL_NAME: &str = "nightly";
/// Name of the env to check if we there is necessity to rerun update
/// process as a root user.
pub const LIVEBOAT_UPDATE_BIN_PATH_ENV: &str = "LIVEBOAT_UPDATE_BIN_PATH";
