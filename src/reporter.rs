// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use std::{
    fs::{self as stdFs},
    path::Path,
};

use crate::GraphChoice;
use handlebars::Handlebars;
#[cfg(test)]
use mockall_double::double;
use rust_embed::RustEmbed;
use serde::Serialize;

#[cfg_attr(test, double)]
use crate::utils::fs;

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
        stdFs::create_dir_all(output)?;
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

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use super::write_report;
    use crate::{utils, GraphChoice};
    use std::path::{Path, PathBuf};

    #[test]
    fn write_edge_report() {
        let output = Path::new("/tmp/foo");

        let mock = utils::mock_fs::write_context();
        mock.expect::<PathBuf, String>()
            .with(
                predicate::function(move |x: &PathBuf| x.eq(&output.join("edge-bundling.html"))),
                predicate::always(),
            )
            .return_once(|_, _| Ok(()));

        let result =  write_report(output, GraphChoice::Edge);
        assert!(result.is_ok())
    }

    #[test]
    fn write_force_directed_report() {
        let output = Path::new("/tmp/foo");

        let mock = utils::mock_fs::write_context();
        mock.expect::<PathBuf, String>()
            .with(
                predicate::function(move |x: &PathBuf| x.eq(&output.join("force-directed.html"))),
                predicate::always(),
            )
            .return_once(|_, _| Ok(()));

        let result = write_report(output, GraphChoice::Directed);
        assert!(result.is_ok())
    }
}
