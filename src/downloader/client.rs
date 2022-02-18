use std::path::{Path, PathBuf};

use async_std::fs::File;
use async_std::io::Cursor;
use futures_util::{AsyncWriteExt, StreamExt};
use indicatif::ProgressBar;
use reqwest::header::ACCEPT;
use reqwest::Response;

struct FileInfo {
    final_path: PathBuf,
}

impl FileInfo {
    fn new(url: &str, path: &Path, response: &Response) -> Self {
        let file_name = url.split('/').next_back().unwrap();
        let file_name = file_name.split('?').next().unwrap();
        let final_path = path.join(file_name);

        let total_size_option = response.content_length();

        match total_size_option {
            Some(size) => size,
            None => panic!("no response length!"),
        };

        FileInfo { final_path }
    }
}

pub async fn download(
    url: &str,
    path: &Path,
    progress_bar: &ProgressBar,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(ACCEPT, "application/pdf")
        .send()
        .await?;

    let file_info = FileInfo::new(url, path, &response);

    let mut stream = response.bytes_stream();

    let mut file = File::create(format!("{}", file_info.final_path.display())).await?;

    while let Some(chunk) = stream.next().await {
        let chunk_data = chunk.unwrap();

        let mut content = Cursor::new(chunk_data);
        async_std::io::copy(&mut content, &mut file).await?;
    }
    file.close().await?;

    progress_bar.inc(1);
    Ok(())
}
