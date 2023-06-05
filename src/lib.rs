mod chart;
mod cli;
mod reporter;
mod utils;

use anyhow::Result;
pub use cli::{Cli, GraphChoice};
use log::{debug, info};
use pact_broker_api::client::Builder;
use pact_broker_models::contract::Contract;
use reqwest::Url;
use std::{path::Path, time::Duration};

use crate::chart::dataset;

pub async fn run(args: Cli) -> Result<()> {
    info!("Base URL: {}", args.url);
    let output = Path::new(&args.output);
    info!("Output: {}", output.display());
    debug!("Exclude: {:?}", &args.exclude);

    let timeout = Duration::from_millis(args.timeout as u64);

    let mut api_builder = Builder::new()
        .base_url(args.url)
        .unwrap()
        .with_timeout(timeout);

    if let (Some(username), Some(password)) = (args.username, args.password) {
        api_builder = api_builder.basic_auth(&username, &password);
    }

    if let Some(token) = args.token {
        api_builder = api_builder.token(&token);
    }

    let api = api_builder.build().unwrap();
    let urls: Vec<Url> = match api.pacts().latest().await {
        Ok(pacts) => pacts
            .pacts
            .iter()
            .filter_map(|pact| match pact.links.links_self.first() {
                Some(link) => match Url::parse(link.href.as_str()) {
                    Ok(link) => Some(link),
                    Err(_) => None,
                },
                None => None,
            })
            .collect(),
        Err(e) => {
            eprintln!("Failed to fetch latests pacts:");
            return Err(e.into());
        }
    };

    let data = match api.batch_get::<Contract>(urls, None).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to fetch iterate over contracts:");
            return Err(e.into());
        }
    };

    let graph = dataset::Graph::from(&data);
    let json_data = serde_json::to_string(&graph)?;
    reporter::write_report(output, args.graph, json_data).expect("Could not generate the report");
    Ok(())
}
