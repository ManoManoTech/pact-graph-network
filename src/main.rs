// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

extern crate clap;

use clap::Parser;
use env_logger::Target;
use log::info;

use pact_graph_network::{run, Cli};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    info!("Parsing command line argument");
    let cli = Cli::parse();
    run(cli).await?;

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
