use std::fs::File;
use std::io::Write;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    url: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::from_args();

    println!("Download from url: {}", args.url);
    let response = reqwest::get(args.url)
        .await?;

    let total_size_option = response.content_length();

    let total_size = match total_size_option {
        Some(size) => size,
        None => panic!("no response length!"),
    };

    let mut stream = response.bytes_stream();
    println!("Saving file to: {}", &args.path.display());

    let mut file = File::create("foo.txt")?;
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

    bar.finish_with_message(format!("File saved under {} 📦", &args.path.display()));
    Ok(())
}
