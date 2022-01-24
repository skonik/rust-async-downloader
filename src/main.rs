use std::path::PathBuf;

use futures_util::future::join_all;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use structopt::StructOpt;
use std::fs::File;
use std::io::prelude::*;
mod cli;
mod downloader;

async fn process_urls(urls: &Vec<String>, path: &std::path::PathBuf) {
    let progress_bar = ProgressBar::new(urls.len() as u64);
    progress_bar.set_message("Downloading files progress");

    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({per_sec}, {eta})")
        .progress_chars("#>-"));

    let mut futures = Vec::new();
    for url in urls {
        futures.push(downloader::client::download(
            &url,
            &path,
            &progress_bar,
        ));
    }

    let joined_futures = join_all(futures);
    joined_futures.await;

    progress_bar.finish_with_message(format!(
        "Files are saved. 📦",
    ));
}

fn split_into_urls(content: &String, delimiter: char) -> Vec<String> {
    let urls_divided: Vec<String> = content.split(delimiter).map(|s| s.to_string()).collect();

    return urls_divided;
}

fn read_urls_from_file(urls_file: &std::path::PathBuf) -> Vec<String> {
    let mut file = File::open(urls_file).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    return split_into_urls(&contents, '\n');
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::args::Cli::from_args();
    println!("Arguments parsed: \n {}", args);

    let urls: Vec<String> = read_urls_from_file(&args.urls_file_path);

    process_urls(&urls, &args.result_dir_path).await;

    Ok(())
}
