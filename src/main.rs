#![allow(dead_code)]

use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use error::{Error, Result};

mod cli;
mod error;

#[derive(Debug)]
struct Config {
    lockpick_dir: PathBuf,
}

impl Config {
    fn is_valid_path(path_buf: PathBuf) -> Result<bool> {
        if !path_buf.exists() {
            return Err(Error::from("File not found"));
        }

        let extension = path_buf.extension();
        match extension {
            Some(ext) => {
                if ext != "json" {
                    let output = format!(
                        "Config file extension should be {:?}, not {:?}",
                        "json", ext
                    );
                    return Err(Error::from(output));
                }
            },
            None => {
                let output = format!("Path to file expected, got {:?}", path_buf);
                return Err(Error::from(output));
            },
        };

        Ok(true)
    }

    pub fn from_json(path_buf: PathBuf) -> Result<Self> {
        assert!(Self::is_valid_path(path_buf)?);

        let config = Self {
            lockpick_dir: PathBuf::from("./path/to/lockpick"),
        };

        return Ok(config);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { lockpick_dir: PathBuf::from("../lockpick") }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = match cli.config {
        Some(path_buf) => Config::from_json(path_buf)?,
        None => Config::default(),
    };

    println!("{:?}", config);

    Ok(())
}
