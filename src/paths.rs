use rand::{distributions::Alphanumeric, Rng};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use resolve_path::PathResolveExt;

use libnewsboat::configpaths::ConfigPaths as NConfig;
use libnewsboat::configpaths::NEWSBOAT_CONFIG_SUBDIR;

use crate::args::Args;

const LIVEBOAT_CONFIG_FILENAME: &str = "liveboat_config.toml";
const LIVEBOAT_BUILD_DIRNAME: &str = "build";
const LIVEBOAT_CONFIG_DIRNAME: &str = ".config/liveboat";
const LIVEBOAT_TEMPLATES_DIRNAME: &str = "templates";

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
    /// Path to Newsboat lock file.
    lock_file: PathBuf,
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
    pub fn new(args: &Args) -> Result<Paths, Box<dyn Error>> {
        let n_config = NConfig::new();

        if !n_config.initialized() {
            return Err(n_config.error_message().into());
        };

        let mut paths = Paths {
            cache_file: path_with_argval(
                &args.cache_file,
                true,
                n_config.cache_file().to_path_buf(),
            )?,
            url_file: path_with_argval(&args.url_file, true, n_config.url_file().to_path_buf())?,
            lock_file: n_config.lock_file().to_path_buf(),
            config_file: PathBuf::new(),
            build_dir: PathBuf::new(),
            tmp_dir: PathBuf::new(),
            template_dir: PathBuf::new(),
            template_path: PathBuf::new(),
            config_dir: PathBuf::new(),
        };

        paths.config_dir = paths.home().join(LIVEBOAT_CONFIG_DIRNAME);
        paths.template_dir = paths.config_dir.join(LIVEBOAT_TEMPLATES_DIRNAME);

        paths.config_file = path_with_argval(
            &args.config_file,
            false,
            paths.config_dir.join(LIVEBOAT_CONFIG_FILENAME),
        )?;

        paths.build_dir = path_with_argval(
            &args.build_dir,
            false,
            paths.home().join(LIVEBOAT_BUILD_DIRNAME),
        )?;
        paths.tmp_dir =
            std::env::temp_dir().join(format!("liveboat-{}", generate_random_string(5)));

        return Ok(paths);
    }

    pub fn initialized(&self) -> bool {
        return self.config_file.is_file();
    }

    pub fn lock_exists(&self) -> bool {
        return self.lock_file.is_file();
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


    pub fn set_template_path(&mut self, p: &Option<String>, default: &String) -> Result<(), Box<dyn Error>> {
        self.template_path = path_with_argval(
            p,
            true,
            self.template_dir().join(default),
        )?;
        Ok(())
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

fn path_with_argval(
    arg: &Option<String>,
    check_exists: bool,
    default: PathBuf,
) -> Result<PathBuf, String> {
    if let Some(argval) = arg {
        match fs::canonicalize(&argval.resolve()) {
            Err(_) => {
                // TODO: log nonexistent path otherwise
                if check_exists {
                    return Err(format!("Path {} does not exist!", &argval));
                }
            }
            Ok(p) => {
                return Ok(p)
            }
        }
    }
    Ok(default)
}
