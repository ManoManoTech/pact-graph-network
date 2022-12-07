// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

extern crate clap;

use std::path::Path;

use clap::Parser;
use env_logger::Target;
use log::info;

use crate::broker_service::BrockerService;

mod broker_service;
mod chart;
mod client;
mod models;
mod reporter;

#[derive(Debug, Clone, clap::ValueEnum, Copy)]
pub enum GraphChoice {
    Edge,
    Directed,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    /// Pact broker URL
    #[arg(short, long)]
    url: String,
    /// Path of the output dir
    #[arg(short, long, default_value = "report")]
    output: String,
    // #[clap(short, long, parse(from_occurrences))]
    // verbosity: usize,
    #[arg(short, long, value_enum, default_value = "edge")]
    graph: GraphChoice,
}

impl Cli {
    async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Base URL: {}", self.url);
        let output = Path::new(&self.output);
        info!("Output: {}", output.display());

        reporter::write_report(output, self.graph).expect("Could not generate the report");

        let grapher: Box<dyn chart::Writer> = match self.graph {
            GraphChoice::Edge => Box::new(chart::EdgeChart::default()),
            GraphChoice::Directed => Box::new(chart::ForceDirectedChart::default()),
        };

        let bs = BrockerService::new(self.url, grapher)?;
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
