use std::{
    io::{self, Write},
    path::Path,
    sync::OnceLock,
};

use owo_colors::OwoColorize;
use tokio::{fs, process::Command};

use crate::Settings;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init_settings(settings: Settings) {
    let _ = SETTINGS.set(settings);
}

pub fn settings() -> &'static Settings {
    SETTINGS.get().expect("settings not initialized")
}

pub async fn create_path(path: &Path) -> bool {
    let confirmation = get_confirmation(
        format!(
            "The path {}, does not exist. Would you like to do this?",
            path.display(),
        )
        .as_str(),
    );

    if confirmation {
        match fs::create_dir_all(path).await {
            Ok(()) => {
                write_suc(format!("Created path: {}", path.display()).as_str());
                return true;
            }
            Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
                let settings = settings();
                let output = Command::new(&settings.superuser_command)
                    .arg("mkdir")
                    .arg("-p")
                    .arg(path)
                    .output()
                    .await
                    .unwrap();

                if output.status.success() {
                    write_suc(format!("Created path (superuser): {}", path.display()).as_str());
                    return true;
                } else {
                    write_err(
                        format!(
                            "Failed, could not create path: {}, Err: {}",
                            path.display(),
                            String::from_utf8_lossy(&output.stderr)
                        )
                        .as_str(),
                    );
                    return false;
                }
            }
            Err(err) => {
                write_err(format!("Failed, could not create path: {}, Err: {}", path.display(), err).as_str());
                return false;
            }
        }
    } else {
        write_err(format!("Aborting, could not create path: {}", path.display(),).as_str());
        return false;
    }
}

pub fn write_err(message: &str) {
    println!("{} {}", "[ERROR]".red(), message);
}

pub fn write_suc(message: &str) {
    println!("{} {}", "[SUCCESS]".green(), message);
}

pub fn get_confirmation(message: &str) -> bool {
    print!("{} {} {}", "[CONFIRM]".bright_purple(), message, "[Y/n] ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            return true;
        }
        "n" | "no" => {
            return false;
        }
        _ => {
            write_err("Input was invalid, assuming no!");
            return false;
        }
    }
}
