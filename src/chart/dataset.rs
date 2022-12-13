use std::collections::HashMap;

use serde::Serialize;

use log::warn;

use crate::client::dto;

#[derive(Debug, Serialize)]
pub struct Graph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

#[derive(Debug, Serialize, Clone)]
struct Node {
    #[serde(rename = "id")]
    index: usize,
    name: String,
    group: Option<String>,
}

impl Node {
    fn new(index: usize, name: String, group: Option<String>) -> Self {
        Self { index, name, group }
    }
}

#[derive(Debug, Serialize)]
struct Link {
    source: usize,
    target: usize,
    #[serde(rename = "type")]
    label: String,
}

impl Link {
    pub fn new(source: usize, target: usize, label: String) -> Self {
        Self {
            source,
            target,
            label,
        }
    }
}

impl From<&Vec<dto::InteractionsResponse>> for Graph {
    fn from(responses: &Vec<dto::InteractionsResponse>) -> Self {
        let mut pacticant = HashMap::<&str, Node>::new();
        let mut links: Vec<Link> = vec![];

        let mut index = 0;
        responses.iter().for_each(|resp| {
            let provider = pacticant
                .entry(&resp.provider.name)
                .or_insert_with(|| {
                    index += 1;
                    Node::new(index, resp.provider.name.clone(), None)
                })
                .clone();
            let consumer = pacticant.entry(&resp.consumer.name).or_insert_with(|| {
                index += 1;
                Node::new(index, resp.consumer.name.clone(), None)
            });
            match &resp.interactions {
                Some(interactions) => interactions.iter().for_each(|interaction| {
                    links.push(Link::new(
                        consumer.index,
                        provider.index,
                        format!(
                            "{} {}",
                            interaction.request.method.to_uppercase(),
                            interaction.request.path
                        ),
                    ))
                }),
                None => warn!(
                    "No interaction describe for {} and {}",
                    consumer.name, provider.name
                ),
            };
        });
        let mut nodes: Vec<Node> = pacticant.values().cloned().collect();
        nodes.sort_by(|a, b| a.index.cmp(&b.index));
        links.sort_by(|a, b| a.source.cmp(&b.source));
        Self { nodes, links }
    }
}
