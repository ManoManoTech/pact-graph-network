// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

mod dependancy;
mod edge_chart;
mod item;

pub use edge_chart::EdgeChart;
pub use item::Item;
use std::{fmt::Error, path::Path};

pub trait Writer {
    fn write(&self, items: Vec<Item>, output: &Path) -> Result<(), Error>;
}
