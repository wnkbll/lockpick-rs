use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Lockpick-rs")]
#[command(version = "0.0.1")]
#[command(about = "Lockpick manager", long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Checks if passed program installed
    Check(CheckArgs),
}

#[derive(Args)]
pub struct CheckArgs {
    name: String,
}
