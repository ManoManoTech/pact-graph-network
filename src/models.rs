use crate::client::dto::{Pact, PactsResponse};
use std::convert::From;

#[derive(Debug)]
pub struct Contract {
    pub consumer: String,
    pub provider: String,
}

impl From<Pact> for Contract {
    fn from(pact: Pact) -> Self {
        Self {
            consumer: pact.embedded.consumer.name,
            provider: pact.embedded.provider.name,
        }
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
