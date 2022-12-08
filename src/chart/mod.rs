// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

mod dataset;
mod edge_chart;
mod force_directed;
use anyhow::Result;

pub use edge_chart::EdgeChart;
pub use force_directed::ForceDirectedChart;
use std::path::Path;

use crate::client::dto;

pub trait Writer {
    fn write(&self, items: &[dto::InteractionsResponse], output: &Path) -> Result<()>;
}
