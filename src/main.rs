use std::fmt;
use std::fs::File;
use std::io::Write;
use std::num::ParseIntError;

use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use structopt::StructOpt;

struct URLParsingError;

impl fmt::Display for URLParsingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "URL has invalid format!")
    }
}

impl fmt::Debug for URLParsingError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{{ file: {}, line: {} }}", file!(), line!())
    }
}


fn validate_url(url: &str) -> Result<String, URLParsingError> {
    let url_regex = Regex::new(r"^https?://.*$").unwrap();

    let result = match url_regex.is_match(url) {
        true => Ok(url.to_string()),
        false => Err(URLParsingError)
    };

    return result;
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(try_from_str = validate_url))]
    url: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();


    let response = reqwest::get(args.url)
        .await?;

    let total_size_option = response.content_length();

    let total_size = match total_size_option {
        Some(size) => size,
        None => panic!("no response length!"),
    };

    let mut stream = response.bytes_stream();
    println!("Saving file to: {}", &args.path.display());

    let mut file = File::create(format!("{}", &args.path.display()))?;
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
