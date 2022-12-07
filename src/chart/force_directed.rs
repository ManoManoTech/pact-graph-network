use std::fs::OpenOptions;

use serde::Serialize;

use super::{dependancy::Dependency, Item};

#[derive(Debug, Serialize)]
struct Node {
    index: usize,
    name: String,
    group: usize,
}

impl From<&Item> for Node {
    fn from(item: &Item) -> Self {
        Self {
            index: item.id(),
            name: item.name().to_owned(),
            group: 1,
        }
    }
}

#[derive(Debug, Serialize)]
struct Graph {
    nodes: Vec<Node>,
    links: Vec<Dependency>,
}

#[derive(Debug, Default)]
pub struct ForceDirectedChart;

impl super::Writer for ForceDirectedChart {
    fn write(&self, items: &[Item], output: &std::path::Path) -> Result<(), std::fmt::Error> {
        let graph = Graph {
            nodes: self.create_nodes(items),
            links: self.create_dependencies(items),
        };

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(output.join("force-directed.json"))
            .unwrap();

        serde_json::to_writer_pretty(file, &graph).expect("Could not write JSON graph");

        Ok(())
    }
}

impl ForceDirectedChart {
    fn create_nodes(&self, items: &[Item]) -> Vec<Node> {
        items.iter().map(Node::from).collect::<Vec<Node>>()
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
}
