use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn download(
    url: String,
    path: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let total_size_option = response.content_length();

    let total_size = match total_size_option {
        Some(size) => size,
        None => panic!("no response length!"),
    };

    let mut stream = response.bytes_stream();
    println!("Saving file to: {}", &path.display());

    let mut file = File::create(format!("{}", &path.display()))?;
    let bar = ProgressBar::new(total_size);

    bar.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));

    let mut downloaded_length: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk_data = chunk.unwrap();

        downloaded_length = downloaded_length + (chunk_data.len() as u64);

        file.write(&chunk_data).or(Err("Chunk writing error"))?;
        bar.set_position(downloaded_length);
    }

    bar.finish_with_message(format!("File saved under {} 📦", &path.display()));
    Ok(())
}
