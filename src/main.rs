mod errors;
use clap::Parser;
use downloader_rs::{cli::Cli, downloader::download_flie};

use crate::errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    download_flie(cli).await?;
    Ok(())
}
