// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use std::{
    fs::{self},
    path::Path,
};

use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde::Serialize;

use crate::GraphChoice;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

#[derive(Debug, Serialize)]
struct SpecData {}

#[derive(Debug, Serialize)]
struct Data {
    spec_data: String,
}

pub fn write_report(output: &Path, graph: GraphChoice) -> Result<(), Box<dyn std::error::Error>> {
    if !output.exists() {
        fs::create_dir_all(output)?;
    }

    let mut hbs = Handlebars::new();
    hbs.register_embed_templates::<Templates>()?;

    let file_name = match graph {
        GraphChoice::Edge => "edge-bundling.html",
        GraphChoice::Directed => "force-directed.html",
    };

    let spec_template = match graph {
        GraphChoice::Edge => "edge-bundling.vg.json",
        GraphChoice::Directed => "force-directed.vg.json",
    };

    let data = Data {
        spec_data: hbs.render(spec_template, &SpecData {})?,
    };

    let html_contents = hbs.render("report.html", &data)?;
    fs::write(output.join(file_name), html_contents)?;

    Ok(())
}
