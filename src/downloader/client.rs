use std::env::args;
use std::path::PathBuf;

use async_std::fs::File;
use async_std::io::Cursor;
use async_std::io::WriteExt;
use futures_util::{AsyncWriteExt, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::header::ACCEPT;
use reqwest::Response;

struct FileInfo {
    total_size: u64,
    file_name: String,
    final_path: PathBuf,
}

impl FileInfo {
    fn new(url: &String, path: &PathBuf, response: &Response) -> Self {
        let file_name = url.split('/').next_back().unwrap();
        let final_path = path.join(file_name);

        let total_size_option = response.content_length();

        let total_size = match total_size_option {
            Some(size) => size,
            None => panic!("no response length!"),
        };

        return FileInfo {
            total_size: total_size,
            file_name: file_name.to_string(),
            final_path: final_path,
        };
    }
}

pub async fn download(
    url: &String,
    path: &std::path::PathBuf,
    progress_bar: &ProgressBar,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(ACCEPT, "application/pdf")
        .send()
        .await?;

    let url_info = FileInfo::new(url, path, &response);

    let mut stream = response.bytes_stream();

    let mut file = File::create(format!("{}", url_info.final_path.display())).await?;



    let mut downloaded_length: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk_data = chunk.unwrap();

        downloaded_length = downloaded_length + (chunk_data.len() as u64);

        let mut content = Cursor::new(chunk_data);
        async_std::io::copy(&mut content, &mut file).await?;
    }
    file.close().await?;

    progress_bar.inc(1);
    Ok(())
}
