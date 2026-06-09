//! Command-line validator for Rusty Makepad settings surfaces.

use std::{env, fs, path::PathBuf, time::SystemTime};

use rusty_makepad_settings::{
    resolve_profile, validate_profile, validate_surface, AppSettingsSurface, SettingsProfile,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(usage());
    };
    let options = parse_options(args.collect())?;
    match command.as_str() {
        "validate-surface" => {
            let surface = read_surface(required_path(&options, "--surface")?)?;
            print_errors(validate_surface(&surface))?;
            if let Some(profile_path) = options.profile.as_ref() {
                let profile = read_profile(profile_path)?;
                print_errors(validate_profile(&surface, &profile))?;
            }
            println!("settings surface validation passed");
            Ok(())
        }
        "resolve" => {
            let surface = read_surface(required_path(&options, "--surface")?)?;
            let profile = read_profile(required_path(&options, "--profile")?)?;
            let generated_at = format!("{:?}", SystemTime::now());
            let report = match resolve_profile(&surface, &profile, 1, generated_at) {
                Ok(report) => report,
                Err(errors) => return Err(format_errors(errors)),
            };
            let text = serde_json::to_string_pretty(&report)
                .map_err(|error| format!("failed to encode report: {error}"))?;
            if let Some(out) = options.out {
                fs::write(out, text).map_err(|error| format!("failed to write report: {error}"))?;
            } else {
                println!("{text}");
            }
            Ok(())
        }
        _ => Err(usage()),
    }
}

#[derive(Default)]
struct Options {
    surface: Option<PathBuf>,
    profile: Option<PathBuf>,
    out: Option<PathBuf>,
}

fn parse_options(args: Vec<String>) -> Result<Options, String> {
    let mut options = Options::default();
    let mut index = 0;
    while index < args.len() {
        let flag = &args[index];
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--surface" => options.surface = Some(PathBuf::from(value)),
            "--profile" => options.profile = Some(PathBuf::from(value)),
            "--out" => options.out = Some(PathBuf::from(value)),
            _ => return Err(format!("unknown option {flag}\n{}", usage())),
        }
        index += 2;
    }
    Ok(options)
}

fn required_path<'a>(options: &'a Options, flag: &str) -> Result<&'a PathBuf, String> {
    match flag {
        "--surface" => options
            .surface
            .as_ref()
            .ok_or_else(|| "--surface is required".to_string()),
        "--profile" => options
            .profile
            .as_ref()
            .ok_or_else(|| "--profile is required".to_string()),
        _ => Err(format!("unsupported required flag {flag}")),
    }
}

fn read_surface(path: &PathBuf) -> Result<AppSettingsSurface, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("failed to read surface {}: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse surface {}: {error}", path.display()))
}

fn read_profile(path: &PathBuf) -> Result<SettingsProfile, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("failed to read profile {}: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse profile {}: {error}", path.display()))
}

fn print_errors(
    result: Result<(), Vec<rusty_makepad_settings::ValidationError>>,
) -> Result<(), String> {
    match result {
        Ok(()) => Ok(()),
        Err(errors) => Err(format_errors(errors)),
    }
}

fn format_errors(errors: Vec<rusty_makepad_settings::ValidationError>) -> String {
    errors
        .into_iter()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn usage() -> String {
    "usage: rusty-makepad-settings-cli validate-surface --surface <path> [--profile <path>]\n       rusty-makepad-settings-cli resolve --surface <path> --profile <path> [--out <path>]".to_string()
}
