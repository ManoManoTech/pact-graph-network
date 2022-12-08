use std::path::Path;

use crate::{broker_service::BrockerService, chart, reporter, GraphChoice};
use clap::Parser;
use log::info;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
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
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
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
