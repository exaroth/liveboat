use std::error::Error;
use std::fs;

use crate::opts::Options;
use crate::paths::Paths;
use crate::args::Args;
use crate::cli;

pub fn cold_start(args: &Args) -> Result<(), Box<dyn Error>> {
    let mut paths = Paths::new(&args.config_file)?;
    paths.update_with_args(args)?;
    let mut opts = Options::default();
    initialization_wizard(&mut opts, &paths)?;
    fs::create_dir_all(paths.template_dir())?;
    opts.save(paths.config_file())?;
    // TODO: download templates to template dir
    Ok(())
}

fn initialization_wizard(opts: &mut Options, paths: &Paths) -> Result<(), Box<dyn Error>> {
    opts.title = cli::prompt_string(opts.title.clone(), "Enter your feed page title:")?;
    opts.newsboat_urls_file =
        cli::prompt_path(&paths.url_file(), true, "Enter path to Newsboat urls file:")?;
    opts.newsboat_cache_file = cli::prompt_path(
        &paths.cache_file(),
        true,
        "Enter path to Newsboat cache db file:",
    )?;
    opts.time_threshold = cli::prompt_int(
        opts.time_threshold,
        "Enter number of days in the past Liveboat should generate feeds for:",
    )?;
    opts.show_read_articles = cli::confirm(
        opts.show_read_articles,
        "Should feed page include articles marked as read by Newsboat?",
    );
    opts.build_dir = cli::prompt_path(
        &paths.build_dir(),
        false,
        "Where should Liveboat save generated pages to?",
    )?;
    Ok(())
}
