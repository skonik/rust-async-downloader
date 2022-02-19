use std::fmt;

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

#[derive(StructOpt)]
#[structopt(
    name = "Rusty Downloader",
    about = "Asynchronously download multiple files."
)]
pub struct Cli {
    #[structopt(parse(from_os_str), short = "u", long = "urls")]
    pub urls_file_path: std::path::PathBuf,
    #[structopt(parse(from_os_str), short = "d", long = "destination")]
    pub result_dir_path: std::path::PathBuf,
    #[structopt(short = "s", long = "silent")]
    pub silent: bool,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "urls file: \n{} \n path: {}",
            self.urls_file_path.display(),
            self.result_dir_path.display()
        )
    }
}
