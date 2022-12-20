use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pacts {
    pub pacts: Vec<Pact>,
    #[serde(rename = "_links")]
    pub links: ResponseLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseLinks {
    #[serde(rename = "self")]
    pub links_self: Link,
    #[serde(rename = "pb:pacts")]
    pub pb_pacts: Vec<PbPact>,
    pub curies: Vec<Cury>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cury {
    pub name: String,
    #[serde(rename = "href")]
    pub href: String,
    pub templated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link {
    pub href: String,
    pub title: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PbPact {
    pub href: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pact {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "_embedded")]
    pub embedded: PactEmbedded,
    #[serde(rename = "_links")]
    pub links: PactLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PactEmbedded {
    pub consumer: Consumer,
    pub provider: Provider,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Consumer {
    pub name: String,
    #[serde(rename = "_embedded")]
    pub embedded: ConsumerEmbedded,
    #[serde(rename = "_links")]
    pub links: ConsumerLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumerEmbedded {
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub number: String,

    #[serde(rename = "_links")]
    pub links: VersionLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionLinks {
    #[serde(rename = "self")]
    pub links_self: Link,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsumerLinks {
    #[serde(rename = "self")]
    pub links_self: Link,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    pub name: String,
    #[serde(rename = "_links")]
    pub links: ConsumerLinks,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PactLinks {
    #[serde(rename = "self")]
    pub links_self: Vec<Link>,
}
