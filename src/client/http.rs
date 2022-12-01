use reqwest::{Error, Url};

use super::dto::pact::PactsResponse;

/// BrokerApi handles communication with pact-broker
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    url: Url,
}

impl Client {
    /// Create a new instance of Broker API service
    pub fn new(url: String) -> Result<Self, Error> {
        let base_url = Url::parse(url.as_str()).expect("Could not parse the base url");
        let client = reqwest::Client::builder()
            .user_agent("pactgrapher")
            .build()?;

        Ok(Self {
            client,
            url: base_url,
        })
    }

    /// Gets a list of contracts
    pub async fn list_pacts(&self) -> Result<PactsResponse, Error> {
        let url = self
            .url
            .join("/pacts/latest")
            .expect(r#"Could not create the entrypoint to get the list of pacts"#);
        let pacts_response = self
            .client
            .get(url)
            .send()
            .await?
            .json::<PactsResponse>()
            .await?;

        Ok(pacts_response)
    }
}
