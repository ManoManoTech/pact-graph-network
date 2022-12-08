// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

extern crate clap;

mod broker_service;
mod chart;
mod cli;
mod client;
mod models;
mod reporter;

use clap::Parser;
use env_logger::Target;
use log::info;

use cli::Cli;

#[derive(Debug, Clone, clap::ValueEnum, Copy)]
pub enum GraphChoice {
    Edge,
    Directed,
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
