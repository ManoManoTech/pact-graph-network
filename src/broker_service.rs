// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use std::collections::HashMap;
use std::convert::From;
use std::fmt::Error;
use std::path::Path;

use serde::Serialize;

use crate::chart::{self, Item};
use crate::client::Client;
use crate::models::{Contract, Contracts};

#[derive(Debug, Serialize)]
struct Dependencies {
    source: u32,
    target: u32,
}

pub struct BrockerService {
    api: Client,
    writer: Box<dyn chart::Writer>,
}

impl BrockerService {
    pub fn new(url: String, writer: Box<dyn chart::Writer>) -> Result<Self, Error> {
        let cli = Client::new(url).expect("Could not initialize the client");
        Ok(Self { api: cli, writer })
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
        let items = create_services(contracts);
        self.writer.write(items, output)?;

        Ok(())
    }
}

fn create_services(contracts: &[Contract]) -> Vec<Item> {
    let mut services: HashMap<String, Item> = HashMap::new();
    let mut id: usize = 0;
    contracts.iter().for_each(|contract| {
        let provider = services
            .entry(contract.provider().to_owned())
            .or_insert_with(|| {
                id += 1;
                Item::new(id, contract.provider().to_owned())
            })
            .clone();

        let consumer = services
            .entry(contract.consumer().to_owned())
            .or_insert_with(|| {
                id += 1;
                Item::new(id, contract.consumer().to_owned())
            });

        let mut updated_consumer = consumer.clone();
        updated_consumer = updated_consumer.add_depends(provider.id());

        services.insert(contract.consumer().to_owned(), updated_consumer);
    });

    let mut items = services.values().cloned().collect::<Vec<Item>>();
    items.sort_by_key(|item| item.id());

    items
}
