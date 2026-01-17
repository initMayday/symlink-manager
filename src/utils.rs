use std::io::{self, Write};

use owo_colors::OwoColorize;

pub fn write_err(message: &str) {
    println!("{} {}", "[ERROR]".red(), message);
}

pub fn write_suc(message: &str) {
    println!("{} {}", "[SUCCESS]".green(), message);
}

pub fn get_confirmation(message: &str) -> bool {
    print!("{} {} {}", "[CONFIRM]".bright_purple(), message, " [Y/n]");
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
