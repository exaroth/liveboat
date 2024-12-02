use rand::{distributions::Alphanumeric, Rng};
use resolve_path::PathResolveExt;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use libnewsboat::configpaths::ConfigPaths as NConfig;

use crate::args::Args;
use crate::errors::FilesystemError;

const LIVEBOAT_CONFIG_FILENAME: &str = "liveboat_config.toml";
const LIVEBOAT_BUILD_DIRNAME: &str = "liveboat_build";
const LIVEBOAT_CONFIG_DIRNAME: &str = ".config/liveboat";
const LIVEBOAT_TEMPLATES_DIRNAME: &str = "templates";

/// This module stores all the paths used by the application.
#[derive(Debug, Default)]
pub struct Paths {
    /// Default path to build directory used to store generated output.
    build_dir: PathBuf,
    /// Path to Liveboat TOML configuration file.
    config_file: PathBuf,
    /// Path to Newsboat cache db file.
    cache_file: PathBuf,
    /// Path to Liveboat config directory
    config_dir: PathBuf,
    /// Path to file containing page templates
    template_dir: PathBuf,
    /// Path to Newsboat urls file.
    url_file: PathBuf,
    /// Path to temporary file used for building the page.
    tmp_dir: PathBuf,
    /// Optional path to template to be used for generating the page
    template_path: PathBuf,
}

fn generate_random_string(len: usize) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
}

impl Paths {

    /// Initialize default paths.
    pub fn new(config_file_path: &Option<String>) -> Result<Paths, FilesystemError> {
        let mut paths = Paths {
            cache_file: PathBuf::new(),
            url_file: PathBuf::new(),
            config_file: PathBuf::new(),
            build_dir: PathBuf::new(),
            tmp_dir: PathBuf::new(),
            template_dir: PathBuf::new(),
            template_path: PathBuf::new(),
            config_dir: PathBuf::new(),
        };

        paths.config_dir = paths.home().join(LIVEBOAT_CONFIG_DIRNAME);
        paths.template_dir = paths.config_dir.join(LIVEBOAT_TEMPLATES_DIRNAME);
        paths.tmp_dir =
            std::env::temp_dir().join(format!("liveboat-{}", generate_random_string(5)));
        paths.config_file = path_with_argval(
            config_file_path,
            false,
            paths.config_dir.join(LIVEBOAT_CONFIG_FILENAME),
        )?;
        paths.build_dir = paths.home().join(LIVEBOAT_BUILD_DIRNAME);

        return Ok(paths);
    }
    
    /// Update paths with those passed by the used when invoking via cli.
    pub fn update_with_args(&mut self, args: &Args) -> Result<(), FilesystemError> {
        let n_config = NConfig::new();

        if !n_config.initialized() {
            return Err(FilesystemError::Unknown(n_config.error_message().into()));
        };
        self.url_file = path_with_argval(&args.url_file, true, self.url_file.clone())?;
        self.cache_file = path_with_argval(&args.cache_file, true, self.cache_file.clone())?;
        self.build_dir = path_with_argval(&args.build_dir, false, self.build_dir.clone())?;
        self.template_path = path_with_argval(
            &args.template_path,
            true,
            self.template_dir().join(self.template_path()),
        )?;
        Ok(())
    }
    
    /// Update with paths as saved in the options file.
    pub fn update_with_opts(
        &mut self,
        url_file: &String,
        cache_file: &String,
        build_dir: &String,
        template_name: &String,
    ) {
        self.url_file = PathBuf::from(url_file);
        self.cache_file = PathBuf::from(cache_file);
        self.build_dir = PathBuf::from(build_dir);
        self.template_path = self.template_dir.join(template_name)
    }
    
    /// Check if all the paths required for app operation are correct.
    pub fn check_all(&self) -> Result<(), FilesystemError> {
        if !self.url_file.is_file() {
            return Err(FilesystemError::PathDoesNotExist(self.url_file.clone()));
        }
        if !self.cache_file.is_file() {
            return Err(FilesystemError::PathDoesNotExist(self.cache_file.clone()));
        }
        if !self.config_file.is_file() {
            return Err(FilesystemError::PathDoesNotExist(self.config_file.clone()));
        }
        if !self.template_path.is_dir() {
            return Err(FilesystemError::PathDoesNotExist(
                self.template_path.clone(),
            ));
        }
        Ok(())
    }
    
    pub fn initialized(&self) -> bool {
        return self.config_file.is_file();
    }

    pub fn url_file(&self) -> &Path {
        return &self.url_file;
    }

    pub fn config_file(&self) -> &Path {
        return &self.config_file;
    }

    pub fn build_dir(&self) -> &Path {
        return &self.build_dir;
    }

    pub fn cache_file(&self) -> &Path {
        return &self.cache_file;
    }

    pub fn tmp_dir(&self) -> &Path {
        return &self.tmp_dir;
    }

    pub fn template_dir(&self) -> &Path {
        return &self.template_dir;
    }
    pub fn template_path(&self) -> &Path {
        return &self.template_path;
    }

    fn home(&self) -> PathBuf {
        #[allow(deprecated)]
        if let Some(home) = std::env::home_dir() {
            return home;
        }
        // NOTE: This clause can only trigger on Windows however
        // since newsboat is not Windows compatible it should never happen
        panic!("Could not retrieve home directory");
    }
}

impl fmt::Display for Paths {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Paths::
            config_file {}:
            config_dir: {}
            tmp_dir: {}
            template_path: {}
            build_dir: {}
            cache_file: {},
            url_file: {}",
            self.config_file.display(),
            self.config_dir.display(),
            self.tmp_dir.display(),
            self.template_path.display(),
            self.build_dir.display(),
            self.cache_file.display(),
            self.url_file.display()
        )
    }
}

/// Set path based on the argument passed by the user
/// (if available), also resolves it to absolute path.
fn path_with_argval(
    arg: &Option<String>,
    check_exists: bool,
    default: PathBuf,
) -> Result<PathBuf, FilesystemError> {
    if let Some(argval) = arg {
        match fs::canonicalize(&argval.resolve()) {
            Err(_) => {
                if check_exists {
                    return Err(FilesystemError::InvalidPathProvided(argval.clone()));
                }
                // TODO: log nonexistent path otherwise
                return Ok(default);
            }
            Ok(p) => return Ok(p),
        }
    }
    Ok(default)
}
