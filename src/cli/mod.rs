use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// URL  from where the file will be downloaded
    #[arg(long)]
    pub url: String,
    /// The directory where the file will be downloaded
    #[arg(short, long, value_name = "path")]
    pub path: Option<PathBuf>,
}
