use std::path::PathBuf;

use futures_util::future::join_all;
use indicatif::{MultiProgress, ProgressBar};
use structopt::StructOpt;

mod cli;
mod downloader;

async fn process_urls(urls: &Vec<String>, path: &std::path::PathBuf) {
    let multi_progress_bar = MultiProgress::new();

    let mut futures = Vec::new();
    for url in urls {
        futures.push(downloader::client::download(
            &url,
            &path,
            &multi_progress_bar,
        ));
    }

    let joined_futures = join_all(futures);
    joined_futures.await;
}

fn split_urls(urls: &String) -> Vec<String> {
    let urls_divided: Vec<String> = urls.split(',').map(|s| s.to_string()).collect();

    return urls_divided;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::args::Cli::from_args();
    println!("Arguments parsed: \n {}", args);

    let urls: Vec<String> = split_urls(&args.url);

    process_urls(&urls, &args.path).await;

    Ok(())
}
