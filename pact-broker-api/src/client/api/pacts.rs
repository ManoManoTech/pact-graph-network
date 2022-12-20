use pact_broker_models::contract::Contract;
use pact_broker_models::pacts::Pacts;

use crate::client::BrokerClient;
use crate::client::Result;

pub struct PactsHandler<'client> {
    client: &'client BrokerClient,
}

impl<'client> PactsHandler<'client> {
    /// Creates a new [`PactsHandler`].
    pub(crate) fn new(client: &'client BrokerClient) -> Self {
        Self { client }
    }

    pub async fn latest(&self) -> Result<Pacts> {
        self.client.get("/pacts/latest", None::<&()>, None).await
    }

    pub async fn contract(
        &self,
        provider: &str,
        consumer: &str,
        version: Option<&str>,
    ) -> Result<Contract> {
        let version = version.unwrap_or("latest");
        let route = format!("/pacts/provider/{provider}/consumer/{consumer}/{version}");
        self.client.get(route, None::<&()>, None).await
    }
}
