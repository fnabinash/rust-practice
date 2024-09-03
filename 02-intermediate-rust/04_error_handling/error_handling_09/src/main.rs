// Create a program that reads a configuration file and handles errors if the file is missing or corrupted.

use std::fs::File;
use std::io::Read;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
struct Config {
    username: String,
    timeout: u64,
    debug_mode: bool,
}

fn read_config_file<P: AsRef<Path>>(path: P) -> Result<Config, String> {
    let mut file = File::open(&path).map_err(|_| format!("Failed to open config file: {:?}", path.as_ref()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| "Failed to read config file contents".to_string())?;

    parse_config(&contents).map_err(|_| "Failed to parse config file (corrupted or invalid format)".to_string())
}

fn parse_config(contents: &str) -> Result<Config, String> {
    let mut username = None;
    let mut timeout = None;
    let mut debug_mode = None;

    for line in contents.lines() {
        let mut parts = line.split('=');
        let key = parts.next().ok_or("Invalid line format")?.trim();
        let value = parts.next().ok_or("Invalid line format")?.trim();

        match key {
            "username" => username = Some(value.to_string()),
            "timeout" => timeout = value.parse().ok(),
            "debug_mode" => debug_mode = value.parse().ok(),
            _ => return Err("Unknown configuration key".to_string()),
        }
    }

    Ok(Config {
        username: username.ok_or("Missing username")?,
        timeout: timeout.ok_or("Missing or invalid timeout")?,
        debug_mode: debug_mode.ok_or("Missing or invalid debug_mode")?,
    })
}

fn main() {
    let config_path = "config.txt";

    match read_config_file(config_path) {
        Ok(config) => {
            println!("Successfully read configuration: {:?}", config);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
