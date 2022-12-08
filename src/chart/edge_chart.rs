// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use super::dataset;
use crate::client::dto;
use anyhow::Result;
use std::{fs::OpenOptions, path::Path};

#[derive(Debug, Default)]
pub struct EdgeChart;

impl super::Writer for EdgeChart {
    fn write(&self, items: &[dto::InteractionsResponse], output: &Path) -> Result<()> {
        let graph = dataset::Graph::from(items);

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output.join("edge-bundling.json"))
            .unwrap();

        serde_json::to_writer_pretty(file, &graph).expect("Could not write JSON graph");

        Ok(())
    }
}
