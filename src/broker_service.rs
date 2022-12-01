use std::collections::{HashMap, VecDeque};
use std::convert::From;
use std::fmt::Error;
use std::fs::OpenOptions;
use std::path::Path;

use serde::Serialize;

use crate::client::Client;
use crate::models::Contracts;

#[derive(Debug, Serialize, Clone)]
struct Item {
    id: u32,
    name: String,
    depends_on: Vec<u32>,
}

#[derive(Debug, Serialize)]
struct Leaf {
    id: u32,
    name: String,
    parent: Option<u32>,
}

impl From<&Item> for Leaf {
    fn from(item: &Item) -> Self {
        Self {
            id: item.id,
            name: item.name.clone(),
            parent: Some(3),
        }
    }
}

#[derive(Debug, Serialize)]
struct Dependencies {
    source: u32,
    target: u32,
}

pub struct BrockerService {
    api: Client,
}

impl BrockerService {
    pub fn new(url: String) -> Result<Self, Error> {
        let cli = Client::new(url).expect("Could not initialize the client");
        Ok(Self { api: cli })
    }

    pub async fn load_contract(&self) -> Result<Contracts, Error> {
        let data = self
            .api
            .list_pacts()
            .await
            .expect("Could not fetch latest pacts from broker.");

        Ok(Contracts::from(data))
    }

    pub fn write(&self, contracts: &Contracts, output: &Path) -> Result<(), Error> {
        let services = create_services(contracts);

        let graph = create_graph_data(services.values().cloned().collect::<Vec<Item>>());
        write_graph_data(&graph, output)?;

        let deps = create_dependencies(services);
        write_dependancies(&deps, output)?;

        Ok(())
    }
}

fn create_services(contracts: &[crate::models::Contract]) -> HashMap<String, Item> {
    let mut services: HashMap<String, Item> = HashMap::new();
    let mut id = 3u32;
    contracts.iter().for_each(|contract| {
        id += 1;
        let provider = services
            .entry(contract.provider.clone())
            .or_insert(Item {
                id,
                name: contract.provider.clone(),
                depends_on: vec![],
            })
            .clone();
        id += 1;
        let consumer = services.entry(contract.consumer.clone()).or_insert(Item {
            id,
            name: contract.consumer.clone(),
            depends_on: vec![],
        });

        let mut updated_consumer = consumer.clone();
        updated_consumer.depends_on.push(provider.id);

        services.insert(contract.consumer.clone(), updated_consumer);
    });
    services
}

fn create_dependencies(services: HashMap<String, Item>) -> Vec<Dependencies> {
    let mut deps: Vec<Dependencies> = vec![];
    services.values().cloned().for_each(|item| {
        item.depends_on.iter().for_each(|target| {
            deps.push(Dependencies {
                source: item.id,
                target: *target,
            })
        })
    });
    deps
}

fn create_graph_data(services: Vec<Item>) -> VecDeque<Leaf> {
    let mut tree = services.iter().map(Leaf::from).collect::<VecDeque<Leaf>>();

    tree.push_front(Leaf {
        id: 3,
        name: String::from("cluster"),
        parent: Some(2),
    });
    tree.push_front(Leaf {
        id: 2,
        name: String::from("analytics"),
        parent: Some(1),
    });
    tree.push_front(Leaf {
        id: 1,
        name: String::from("flare"),
        parent: None,
    });

    tree
}

fn write_dependancies(deps: &Vec<Dependencies>, output: &Path) -> Result<(), Error> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output.join("services-dependencies.json"))
        .unwrap();

    serde_json::to_writer_pretty(file, &deps).expect("Could not write JSON dependencies");

    Ok(())
}

fn write_graph_data(graph_data: &VecDeque<Leaf>, output: &Path) -> Result<(), Error> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output.join("services.json"))
        .unwrap();
    serde_json::to_writer_pretty(file, graph_data).expect("Could not write JSON tree");

    Ok(())
}
