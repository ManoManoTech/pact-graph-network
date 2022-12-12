mod chart;
mod cli;
mod client;
mod reporter;
mod utils;

use anyhow::Result;
pub use cli::{Cli, GraphChoice};
use client::Client;
use log::info;
use std::{path::Path, time::Duration};

pub async fn run(args: Cli) -> Result<()> {
    info!("Base URL: {}", args.url);
    let output = Path::new(&args.output);
    info!("Output: {}", output.display());
    let timeout = Duration::from_millis(args.timeout as u64);

    reporter::write_report(output, args.graph).expect("Could not generate the report");

    let grapher: Box<dyn chart::Writer> = match args.graph {
        GraphChoice::Edge => Box::new(chart::EdgeChart::default()),
        GraphChoice::Directed => Box::new(chart::ForceDirectedChart::default()),
    };

    let api = Client::new(args.url, timeout).expect("Could not initialize the client");
    let data = api
        .pacts()
        .await
        .expect("Could not fetch latest pacts from broker.");
    grapher.write(&data, output)?;
    Ok(())
}
