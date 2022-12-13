// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use std::{
    fs::{self as stdFs},
    path::Path,
};

use crate::utils::fs;
use crate::GraphChoice;
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde::Serialize;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Templates;

#[derive(Debug, Serialize)]
struct Data {
    json_data: String,
}

pub fn write_report(
    output: &Path,
    graph: GraphChoice,
    data: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !output.exists() {
        stdFs::create_dir_all(output)?;
    }

    let mut hbs = Handlebars::new();
    hbs.register_embed_templates::<Templates>()?;

    let template = match graph {
        GraphChoice::Edge => "edge-bundling.hbs",
        GraphChoice::Directed => "force-directed.hbs",
    };

    let data = Data { json_data: data };

    let html_contents = hbs.render(template, &data)?;
    fs::write(
        output.join(template.replace(".hbs", ".html")),
        html_contents,
    )?;

    Ok(())
}
