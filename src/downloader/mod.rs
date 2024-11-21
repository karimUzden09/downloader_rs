use super::errors::Result;
use crate::cli::Cli;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::{cmp::min, io::Write, path::PathBuf};

#[inline]
fn configure_pb(total_size: u64, url: impl Into<String>) -> Result<ProgressBar> {
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url.into()));
    Ok(pb)
}
async fn download_file_internal(url: impl Into<String>, path: Option<PathBuf>) -> Result<()> {
    let url: String = url.into();
    let response = reqwest::get(&url).await?;
    let total_size = response
        .content_length()
        .ok_or(format!("Failed to get content length from {}", url.clone()))?;
    let pb = configure_pb(total_size, &url)?;
    let f_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");
    let f_name_path = path
        .map(|mut a| {
            a.push(f_name);
            a
        })
        .unwrap_or(f_name.into());
    let mut file = std::fs::File::create(&f_name_path)?;
    let mut downloaded = 0_u64;
    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }
    pb.finish_with_message(format!("Downloaded from {} to {:?}", url, f_name_path));
    Ok(())
}

pub async fn download_flie(args: Cli) -> Result<()> {
    download_file_internal(args.url, args.path).await?;
    Ok(())
}
