// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

mod dependancy;
mod edge_chart;
mod force_directed;
mod item;

pub use edge_chart::EdgeChart;
pub use force_directed::ForceDirectedChart;
pub use item::Item;
use std::{fmt::Error, path::Path};

pub trait Writer {
    fn write(&self, items: &[Item], output: &Path) -> Result<(), Error>;
}
