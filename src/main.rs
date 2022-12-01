extern crate clap;

use std::path::Path;

use clap::Parser;
use env_logger::Target;
use log::info;

use crate::broker_service::BrockerService;

mod broker_service;
mod client;
mod models;
mod reporter;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    /// Pact broker URL
    #[arg(short, long)]
    url: String,
    /// Path of the output dir
    #[arg(short, long)]
    output: String,
}

impl Cli {
    async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Base URL: {}", self.url);
        let output = Path::new(&self.output);
        info!("Output: {}", output.display());

        reporter::write_report(output).expect("Could not generate the report");

        let bs = BrockerService::new(self.url)?;
        let contracts = bs.load_contract().await?;
        bs.write(&contracts, output)?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    info!("Parsing command line argument");
    let cli = Cli::parse();
    cli.run().await?;

    Ok(())
}

fn init_logger() {
    env_logger::Builder::from_env(
        env_logger::Env::new()
            .filter("PACT_NETWORK_LOG")
            .write_style("PACT_NETWORK_LOG_STYLE"),
    )
    .target(Target::Stdout)
    .init()
}
