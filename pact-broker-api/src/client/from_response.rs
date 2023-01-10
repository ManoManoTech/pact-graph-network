use super::error;
use super::Result;
use snafu::ResultExt;

/// A trait for mapping from a `reqwest::Response` to an another type.
#[async_trait::async_trait]
pub trait FromResponse: Sized {
    async fn from_response(response: reqwest::Response) -> Result<Self>;
}

#[async_trait::async_trait]
impl<T: serde::de::DeserializeOwned> FromResponse for T {
    async fn from_response(response: reqwest::Response) -> Result<Self> {
        let text = response.text().await.context(error::HttpSnafu)?;

        let de = &mut serde_json::Deserializer::from_str(&text);
        serde_path_to_error::deserialize(de).context(error::JsonSnafu)
    }
}
