use indicatif::{MultiProgress, ProgressBar};
use structopt::StructOpt;

use futures_util::future::join_all;
use std::path::PathBuf;

mod cli;
mod downloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::args::Cli::from_args();

    let multi_progress_bar = MultiProgress::new();

    let urls: Vec<String> = args.url.split(',').map(|s| s.to_string()).collect();

    let mut futures = Vec::new();
    for url in &urls {
        futures.push(downloader::client::download(
            &url,
            &args.path,
            &multi_progress_bar,
        ));
    }

    let joined_futures = join_all(futures);
    joined_futures.await;

    Ok(())
}
