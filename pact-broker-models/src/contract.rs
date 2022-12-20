use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Contract {
    pub consumer: Pacticant,
    pub interactions: Option<Vec<Interaction>>,
    pub metadata: Metadata,
    pub provider: Pacticant,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pacticant {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Interaction {
    #[serde(rename = "_id")]
    pub id: String,
    pub description: String,
    #[serde(rename = "providerStates")]
    pub provider_states: Option<Vec<State>>,
    pub request: Request,
    pub response: Response,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub status: i16,
    pub headers: Option<serde_json::Value>,
    pub body: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    links_self: Link,
    #[serde(rename = "pb:consumer")]
    pb_consumer: Link,
    #[serde(rename = "pb:consumer-version")]
    pb_consumer_version: Link,
    #[serde(rename = "pb:provider")]
    pb_provider: Link,
    #[serde(rename = "pb:pact-version")]
    pb_pact_version: Link,
    #[serde(rename = "pb:latest-pact-version")]
    pb_latest_pact_version: Link,
    #[serde(rename = "pb:all-pact-versions")]
    pb_all_pact_versions: Link,
    #[serde(rename = "pb:latest-untagged-pact-version")]
    pb_latest_untagged_pact_version: Link,
    #[serde(rename = "pb:latest-tagged-pact-version")]
    pb_latest_tagged_pact_version: Link,
    #[serde(rename = "pb:previous-distinct")]
    pb_previous_distinct: Link,
    #[serde(rename = "pb:diff-previous-distinct")]
    pb_diff_previous_distinct: Link,
    #[serde(rename = "pb:diff")]
    pb_diff: Link,
    #[serde(rename = "pb:pact-webhooks")]
    pb_pact_webhooks: Link,
    #[serde(rename = "pb:consumer-webhooks")]
    pb_consumer_webhooks: Link,
    #[serde(rename = "pb:tag-prod-version")]
    pb_tag_prod_version: Link,
    #[serde(rename = "pb:tag-version")]
    pb_tag_version: Link,
    #[serde(rename = "pb:publish-verification-results")]
    pb_publish_verification_results: Link,
    #[serde(rename = "pb:latest-verification-results")]
    pb_latest_verification_results: Link,
    #[serde(rename = "pb:triggered-webhooks")]
    pb_triggered_webhooks: Link,
    #[serde(rename = "pb:matrix-for-consumer-version")]
    pb_matrix_for_consumer_version: Link,
    curies: Vec<Cury>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cury {
    name: String,
    #[serde(rename = "href")]
    href: String,
    #[serde(rename = "templated")]
    templated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    title: Option<String>,
    href: String,
    name: Option<String>,
    templated: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    #[serde(rename = "pactSpecification")]
    pact_specification: Pact,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pact {
    #[serde(rename = "version")]
    version: String,
}
