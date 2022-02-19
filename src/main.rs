use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use structopt::StructOpt;

mod cli;
mod downloader;

async fn process_urls<'a>(urls: &'a [String], path: &'a Path, silent: bool) {
    let downloader_client = downloader::client::DownloaderClient::new(urls, silent, path);
    downloader_client.download_all().await;
}

fn split_into_urls(content: &str, delimiter: char) -> Vec<String> {
    let urls_divided: Vec<String> = content.split(delimiter).map(|s| s.to_string()).collect();

    urls_divided
}

fn read_urls_from_file(urls_file: &Path) -> Vec<String> {
    let mut file = File::open(urls_file).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    split_into_urls(&contents, '\n')
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::args::Cli::from_args();
    let urls: Vec<String> = read_urls_from_file(&args.urls_file_path);
    process_urls(&urls, &args.result_dir_path, args.silent).await;
    Ok(())
}
