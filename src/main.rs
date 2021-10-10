use std::fs;
use std::path;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    url: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::from_args();

    println!("Download from url: {}", args.url);
    let response_bytes = reqwest::blocking::get(args.url)?
        .bytes()?;

    println!("Saving file to: {}", args.path.display());
    fs::write(args.path, response_bytes).expect("Unable to write file");

    Ok(())
}
