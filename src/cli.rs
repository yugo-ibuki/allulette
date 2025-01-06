use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the JSON file containing options
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    /// Number of items to select
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,

    /// List all available options
    #[arg(long)]
    pub list_options: bool,

    /// Show selection history
    #[arg(long)]
    pub history: bool,

    /// Clear selection history
    #[arg(long)]
    pub clear_history: bool,
} 