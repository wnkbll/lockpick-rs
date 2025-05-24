#![allow(dead_code)]

use clap::Parser;
use cli::Cli;
use config::Config;
pub use error::{Error, Result};

mod cli;
mod config;
mod error;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = match cli.config {
        Some(path_buf) => Config::from_json(path_buf)?,
        None => Config::default(),
    };

    println!("{:?}", config);

    Ok(())
}
