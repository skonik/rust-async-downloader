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
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub urls_file_path: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    pub result_dir_path: std::path::PathBuf,
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
