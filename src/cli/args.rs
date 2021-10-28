use regex::Regex;
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

fn validate_url(url: &str) -> Result<String, URLParsingError> {
    let url_regex = Regex::new(r"^https?://.*$").unwrap();

    let result = match url_regex.is_match(url) {
        true => Ok(url.to_string()),
        false => Err(URLParsingError),
    };

    return result;
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(try_from_str = validate_url))]
    pub url: String,
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

impl fmt::Display for Cli {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let url: String = self.url.replace(",", "\n");
        write!(f, "urls: \n{} \n path: {}", url, self.path.display())
    }
}
