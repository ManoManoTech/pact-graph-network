// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use std::{collections::VecDeque, fmt::Error, fs::OpenOptions, path::Path};

use serde::Serialize;

use super::{dependancy::Dependency, Item};

#[derive(Debug, Serialize)]
struct Leaf {
    id: usize,
    name: String,
    parent: Option<usize>,
}

impl From<&Item> for Leaf {
    fn from(item: &Item) -> Self {
        Self {
            id: item.id(),
            name: item.name().to_owned(),
            parent: Some(3),
        }
    }
}

#[derive(Debug, Default)]
pub struct EdgeChart;

impl super::Writer for EdgeChart {
    fn write(&self, items: Vec<super::Item>, output: &Path) -> Result<(), Error> {
        let graph = self.create_graph_data(&items);
        self.write_graph_data(&graph, output)?;
        let deps = self.create_dependencies(&items);
        self.write_dependancies(&deps, output)?;

        Ok(())
    }
}

impl EdgeChart {
    fn create_graph_data(&self, items: &[Item]) -> VecDeque<Leaf> {
        let size = items.len();
        let cluster_id = size + 3;
        let mut tree = items
            .iter()
            .map(|item| {
                let mut leaf = Leaf::from(item);
                leaf.parent = Some(cluster_id);
                leaf
            })
            .collect::<VecDeque<Leaf>>();

        tree.push_front(Leaf {
            id: cluster_id,
            name: String::from("cluster"),
            parent: Some(cluster_id - 1),
        });
        tree.push_front(Leaf {
            id: cluster_id - 1,
            name: String::from("analytics"),
            parent: Some(cluster_id - 2),
        });
        tree.push_front(Leaf {
            id: cluster_id - 2,
            name: String::from("flare"),
            parent: None,
        });

        tree
    }

    fn create_dependencies(&self, items: &[Item]) -> Vec<Dependency> {
        let mut deps: Vec<Dependency> = vec![];
        items.iter().for_each(|item| {
            item.depends_on().iter().for_each(|target| {
                deps.push({
                    let source = item.id();
                    Dependency::new(source, *target)
                })
            })
        });
        deps
    }

    fn write_dependancies(&self, deps: &[Dependency], output: &Path) -> Result<(), Error> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output.join("services-dependencies.json"))
            .unwrap();

        serde_json::to_writer_pretty(file, &deps).expect("Could not write JSON dependencies");

        Ok(())
    }

    fn write_graph_data(&self, graph_data: &VecDeque<Leaf>, output: &Path) -> Result<(), Error> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output.join("services.json"))
            .unwrap();
        serde_json::to_writer_pretty(file, graph_data).expect("Could not write JSON tree");

        Ok(())
    }
}
