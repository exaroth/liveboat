use resolve_path::PathResolveExt;
use std::error::Error;
use log::info;

use console::Style;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use std::path::Path;

/// Prompt for input containing string value.
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

/// Prompt for input containing path.
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
        info!("Resolved path: {}", resolved.display());
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

/// Prompt for input containing integer.
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

/// Prompt for confirmation.
pub fn confirm(default: bool, prompt: &str) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()
        .unwrap()
}
