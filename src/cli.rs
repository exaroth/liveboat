// liveboat -x init
// check if config dir exists and config file exists
// if user passed custom config file check only dir
// if not exist prompt user to run liveboat -x init
// if not found prompt user
// prompt for feed title
// prompt for path to newsboat urls -> check
// prompt for path to newsboat cache -> check
// prompt for build dir path
// prompt whether to show read articles
// prompt for time threshold
use resolve_path::PathResolveExt;
use std::error::Error;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use std::path::Path;

pub fn prompt_string(default: String, prompt: &str) -> Result<String, Box<dyn Error>> {
    let theme: ColorfulTheme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };
    let result = Input::with_theme(&theme)
        .with_prompt(prompt)
        .default(default)
        .interact()?;
    return Ok(result);
}

pub fn prompt_path(
    default: &Path,
    check_exists: bool,
    prompt: &str,
) -> Result<String, Box<dyn Error>> {
    let theme: ColorfulTheme = ColorfulTheme {
        values_style: Style::new().green().dim(),
        ..ColorfulTheme::default()
    };
    let p = String::from(default.to_str().unwrap());
    loop {
        let prompt_result = Input::with_theme(&theme)
            .with_prompt(prompt)
            .default(p.clone())
            .interact()?;

        let resolved = prompt_result.resolve();
        if check_exists {
            if !resolved.exists() {
                println!(
                    "File at path {} does not exist, check path and try again.",
                    resolved.display()
                );
                continue;
            }
        }
        return Ok(resolved.display().to_string());
    }
}

pub fn prompt_int(default: u64, prompt: &str) -> Result<u64, Box<dyn Error>> {
    loop {
        let prompt_result = prompt_string(default.to_string(), prompt)?;
        match prompt_result.parse::<u64>() {
            Err(_) => {
                println!("Invalid value");
                continue;
            }
            Ok(i) => return Ok(i),
        }
    }
}

pub fn confirm(default: bool, prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()
        .unwrap()
}
