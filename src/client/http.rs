use std::{sync::mpsc, time::Duration};

use anyhow::Result;
use futures::{stream, StreamExt};
use log::{debug, error, info, trace};
use reqwest::{Error, Url};

use super::dto::{pacts::PactsResponse, InteractionsResponse};

const BUFFER_SIZE: usize = 100;

/// BrokerApi handles communication with pact-broker
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    url: Url,
}

impl Client {
    /// Create a new instance of Broker API service
    pub fn new(url: String, timeout: Duration) -> Result<Self, Error> {
        let base_url = Url::parse(url.as_str()).expect("Could not parse the base url");
        let client = reqwest::Client::builder()
            .user_agent("pactgrapher")
            .timeout(timeout)
            .build()?;

        Ok(Self {
            client,
            url: base_url,
        })
    }

    /// Gets a list of contracts
    pub async fn pacts(&self) -> Result<Vec<InteractionsResponse>> {
        info!("Fetch list of contracts");
        let url = self
            .url
            .join("/pacts/latest")
            .expect(r#"Could not create the entrypoint to get the list of pacts"#);
        debug!("{}", url);
        let pacts_response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PactsResponse>()
            .await?;

        let urls = contracts_url_from(&pacts_response);

        let contracts = self.load_interactions(urls).await?;

        Ok(contracts)
    }

    async fn load_interactions(&self, urls: Vec<Url>) -> Result<Vec<InteractionsResponse>> {
        info!("Fetch interactions");
        let contracts = stream::iter(urls)
            .map(|url| {
                let client = &self.client;
                async move {
                    trace!("load contract: {}", url);
                    let resp = client.get(url).send().await?;
                    resp.json::<InteractionsResponse>().await
                }
            })
            .buffer_unordered(BUFFER_SIZE);
        debug!("Waiting for loading all interactions");

        let (tx, rx) = mpsc::channel::<InteractionsResponse>();

        contracts
            .for_each(|contract| {
                let sender = tx.clone();
                async move {
                    match contract {
                        Ok(contract) => {
                            debug!(
                                "contract loaded between {} and {}",
                                contract.consumer.name, contract.provider.name
                            );
                            if let Err(e) = sender.send(contract) {
                                error!("Could not send contrat to channel: {}", e)
                            }
                        }
                        Err(e) => {
                            error!("could not load the contract: {}", e);
                        }
                    }
                }
            })
            .await;
        drop(tx);
        let mut interactions: Vec<InteractionsResponse> = vec![];
        for interaction in rx {
            trace!(
                "receive interaction between {} and {}",
                interaction.consumer.name,
                interaction.provider.name
            );
            interactions.push(interaction);
        }
        debug!("interactions: {}", interactions.len());
        Ok(interactions)
    }
}

fn contracts_url_from(pacts_response: &PactsResponse) -> Vec<Url> {
    let urls = pacts_response
        .pacts
        .iter()
        // .cloned()
        .filter_map(|p| match p.links.links_self.first() {
            Some(element) => match Url::parse(element.href.as_str()) {
                Ok(link) => Some(link),
                Err(_) => None,
            },
            None => None,
        })
        .collect::<Vec<Url>>();
    urls
}
