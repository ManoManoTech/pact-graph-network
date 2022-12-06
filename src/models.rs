// Copyright 2022 ManoMano Colibri SAS.
// SPDX-License-Identifier: MIT

use crate::client::dto::{Pact, PactsResponse};
use std::convert::From;

#[derive(Debug)]
pub struct Contract(String, String);

impl Contract {
    /// Creates a new [`Contract`].
    pub fn new(consumer: String, provider: String) -> Self {
        Self(consumer, provider)
    }

    /// Returns a reference to the consumer of this [`Contract`].
    pub fn consumer(&self) -> &str {
        self.0.as_ref()
    }

    /// Returns a reference to the provider of this [`Contract`].
    pub fn provider(&self) -> &str {
        self.1.as_ref()
    }
}

impl From<Pact> for Contract {
    fn from(pact: Pact) -> Self {
        Self(pact.embedded.consumer.name, pact.embedded.provider.name)
    }
}

pub type Contracts = Vec<Contract>;

impl From<PactsResponse> for Contracts {
    fn from(data: PactsResponse) -> Self {
        let mut contracts = Self::new();
        for pact in data.pacts {
            contracts.push(Contract::from(pact))
        }
        contracts
    }
}

#[cfg(test)]
mod test {
    use super::Contract;

    #[test]
    fn create_contract() {
        let contract = Contract::new("foo".to_owned(), "bar".to_owned());
        assert_eq!(String::from("foo"), contract.consumer());
        assert_eq!(String::from("bar"), contract.provider());
    }
}
