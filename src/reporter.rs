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

        let result = write_report(output, GraphChoice::Edge, "fake".to_owned());
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

        let result = write_report(output, GraphChoice::Directed, "fake".to_owned());
        assert!(result.is_ok())
    }
}
