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

pub async fn create_path(path: &Path) -> io::Result<()> {
    match fs::create_dir_all(path).await {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::PermissionDenied => {
            let settings = settings();
            let output = Command::new(&settings.superuser_command)
                .arg("mkdir")
                .arg("-p")
                .arg(path)
                .output()
                .await?;

            if output.status.success() {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "superuser mkdir failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ),
                ))
            }
        }
        Err(err) => Err(err),
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
