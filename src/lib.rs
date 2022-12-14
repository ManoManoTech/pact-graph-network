mod chart;
mod cli;
mod client;
mod reporter;
mod utils;

use anyhow::Result;
use chart::dataset;
pub use cli::{Cli, GraphChoice};
use client::Client;
use log::{debug, info};
use std::{path::Path, time::Duration};

pub async fn run(args: Cli) -> Result<()> {
    info!("Base URL: {}", args.url);
    let output = Path::new(&args.output);
    info!("Output: {}", output.display());
    debug!("Exclude: {:?}", &args.exclude);

    let timeout = Duration::from_millis(args.timeout as u64);

    let api = Client::new(args.url, timeout).expect("Could not initialize the client");
    let data = api
        .pacts(args.exclude)
        .await
        .expect("Could not fetch latest pacts from broker.");

    let graph = dataset::Graph::from(&data);
    let json_data = serde_json::to_string(&graph)?;
    reporter::write_report(output, args.graph, json_data).expect("Could not generate the report");
    Ok(())
}
