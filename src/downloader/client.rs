use std::path::{Path, PathBuf};

use async_std::fs::File;
use async_std::io::Cursor;
use futures_util::future::join_all;
use futures_util::{AsyncWriteExt, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
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

pub struct DownloaderClient<'a> {
    pub progress_bar: Option<ProgressBar>,
    pub urls: &'a [String],
    pub silent: bool,
    pub path: &'a Path,
}

impl<'a> DownloaderClient<'a> {
    pub fn new(urls: &'a [String], silent: bool, path: &'a Path) -> Self {
        if !silent {
            let progress_bar = ProgressBar::new(urls.len() as u64);
            progress_bar.set_message("Downloading files progress");
            progress_bar.set_style(ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({per_sec}, {eta})")
                .progress_chars("#>-"));

            Self {
                progress_bar: Option::Some(progress_bar),
                urls,
                silent,
                path,
            }
        } else {
            Self {
                progress_bar: Option::None,
                urls,
                silent,
                path,
            }
        }
    }

    pub async fn download_all(self) {
        let mut futures = Vec::new();
        for url in self.urls {
            futures.push(self.download_single(url, self.path));
        }

        let joined_futures = join_all(futures);
        joined_futures.await;

        if !self.silent {
            self.progress_bar
                .as_ref()
                .unwrap()
                .finish_with_message("Files are saved. 📦");
        }
    }

    pub async fn download_single(
        &self,
        url: &str,
        path: &Path,
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

        match self.silent {
            true => {}
            false => self.progress_bar.as_ref().unwrap().inc(1),
        }

        Ok(())
    }
}
