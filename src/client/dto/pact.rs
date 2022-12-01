use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pact {
    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
}

#[derive(Deserialize, Debug)]
pub struct Embedded {
    pub consumer: Service,
    pub provider: Service,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct PactsResponse {
    pub pacts: Vec<Pact>,
}
