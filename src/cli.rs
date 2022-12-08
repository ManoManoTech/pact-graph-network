use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    static ref VERSION: &'static str =
        option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT").unwrap_or(env!("VERGEN_BUILD_SEMVER"));
    static ref LONG_VERSION: String = format!(
        "
Build Timestamp:     {}
Build Version:       {}
Commit SHA:          {:?}
Commit Date:         {:?}
Commit Branch:       {:?}
",
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_BUILD_SEMVER"),
        option_env!("VERGEN_GIT_SHA"),
        option_env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
        option_env!("VERGEN_GIT_BRANCH")
    );
}

#[derive(Debug, Clone, clap::ValueEnum, Copy)]
pub enum GraphChoice {
    Edge,
    Directed,
}

#[derive(Debug, Parser)]
#[command(
    author,
    version(*VERSION),
    long_version(LONG_VERSION.as_str()),
    about,
    long_about,
)]
pub struct Cli {
    /// Pact broker URL
    #[arg(short, long)]
    pub url: String,
    /// Path of the output dir
    #[arg(short, long, default_value = "report")]
    pub output: String,
    // #[clap(short, long, parse(from_occurrences))]
    // verbosity: usize,
    #[arg(short, long, value_enum, default_value = "edge")]
    pub graph: GraphChoice,
}
