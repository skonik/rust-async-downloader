use std::env::args;

use async_std::fs::File;
use async_std::io::WriteExt;
use futures_util::{AsyncWriteExt, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub async fn download(
    url: &String,
    path: &std::path::PathBuf,
    multi_progress: &MultiProgress,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let file_name = url.split('/').next_back().unwrap();
    let final_path = path.join(file_name);

    let total_size_option = response.content_length();

    let total_size = match total_size_option {
        Some(size) => size,
        None => panic!("no response length!"),
    };

    let mut stream = response.bytes_stream();

    let mut file = File::create(format!("{}", &final_path.display())).await?;
    let bar = multi_progress.add(ProgressBar::new(total_size));

    bar.set_message(file_name.to_string());

    bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));

    let mut downloaded_length: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk_data = chunk.unwrap();

        downloaded_length = downloaded_length + (chunk_data.len() as u64);

        file.write(&chunk_data).await?;
        bar.set_position(downloaded_length);
    }
    file.close().await?;
    bar.finish_with_message(format!("File saved under {} 📦", &final_path.display()));
    Ok(())
}
