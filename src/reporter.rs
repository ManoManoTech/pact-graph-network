// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use std::{
    fs::{self},
    io::{self},
    path::Path,
};

use rust_embed::RustEmbed;

pub fn write_report(output: &Path) -> Result<(), io::Error> {
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output)?;
    let report = Templates::get("report.html").expect("Could not load the report template");
    let html_path = output.join("report.html");
    fs::write(html_path, &report.data)?;

    let graph =
        Templates::get("edge-bundling.vg.json").expect("Could not load the report template");
    let graph_path = output.join("edge-bundling.vg.json");
    fs::write(graph_path, &graph.data)?;

    Ok(())
}

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;
