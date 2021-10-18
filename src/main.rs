use structopt::StructOpt;

mod cli;
mod downloader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::args::Cli::from_args();
    downloader::client::download(args.url, args.path).await?;
    Ok(())
}
