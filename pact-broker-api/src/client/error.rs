use snafu::{Backtrace, ErrorCompat, Snafu};

#[derive(Snafu, Debug)]
#[snafu(visibility(pub))]
pub enum Error {
    Url {
        source: url::ParseError,
        backtrace: Backtrace,
    },
    #[snafu(display("HTTP Error: {}", source))]
    Http {
        source: reqwest::Error,
        #[snafu(backtrace)]
        backtrace: Option<Backtrace>,
    },
    #[snafu(display("Serde Error: {}", source))]
    Serde {
        source: serde_json::Error,
        #[snafu(backtrace)]
        backtrace: Option<Backtrace>,
    },
    #[snafu(display("JSON Error in {}: {}", source.path(), source.inner()))]
    Json {
        source: serde_path_to_error::Error<serde_json::Error>,
        #[snafu(backtrace)]
        backtrace: Option<Backtrace>,
    },
}

impl Error {
    pub fn display_backtrace(&self, verbose: bool) {
        if verbose {
            if let Some(backtrace) = self.backtrace() {
                eprintln!("{}", backtrace);
            }
        }
    }
}
