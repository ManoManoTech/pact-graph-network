use super::{api, auth::Auth, error, from_response::FromResponse, Result};
use futures::{stream, StreamExt};
use log::error;
use reqwest::StatusCode;
use serde::Serialize;
use snafu::ResultExt;
use std::{result::Result::Ok, sync::mpsc};
#[cfg(doctest)]
use tokio_test;
use url::Url;

const MAX_RETRIES: u32 = 3;

/// The Pact Brocker API client.
#[derive(Debug, Clone)]
pub struct BrokerClient {
    pub(crate) client: reqwest::Client,
    pub base_url: Url,
    pub(crate) auth: Auth,
}

/// # Pact Broker API Methods
impl BrokerClient {
    /// Create a new [`api::PactsHandler`].
    pub fn pacts(&self) -> api::PactsHandler {
        api::PactsHandler::new(self)
    }
}

/// # HTTP Methods
/// A collection of different of HTTP methods to use with BrokerClient.
/// All of the HTTP methods (`get`, `post`, etc.) perform some of pre-proceseeing
/// such as making relative urls.absolute, and post processing such as mapping errors,
/// and deserializing the response body.
impl BrokerClient {
    /// Send a `GET` request with optional query parameters, and optional extra header
    /// returning the body of the response.
    pub async fn get<A, P, R>(
        &self,
        route: A,
        parameters: Option<&P>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize + ?Sized,
        R: FromResponse,
    {
        let url = self.absolute_url(route)?;
        let mut request = self.client.get(url);
        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }
        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        let response = self.execute(request).await?;
        R::from_response(response).await
    }

    /// Send multiple `GET` requests with optional header
    /// returning a vector of responses.
    pub async fn batch_get<R>(
        &self,
        urls: Vec<Url>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<Vec<R>>
    where
        R: FromResponse,
    {
        let items = stream::iter(urls)
            .map(|url| {
                let this = &self;
                let headers = headers.clone();
                async move {
                    let mut request = this.client.get(url);
                    if let Some(headers) = headers {
                        request = request.headers(headers);
                    }

                    let response = this.execute(request).await?;
                    R::from_response(response).await
                }
            })
            .buffer_unordered(100);

        let (tx, rx) = mpsc::channel::<R>();

        items
            .for_each(|item| {
                let sender = tx.clone();
                async move {
                    match item {
                        Ok(item) => {
                            if let Err(e) = sender.send(item) {
                                error!("Could not send item to the channel: {:#?}", e)
                            }
                        }
                        Err(e) => error!("Could not get resources: {:#?}", e),
                    }
                }
            })
            .await;
        drop(tx);

        let mut result: Vec<R> = vec![];

        for item in rx {
            result.push(item);
        }

        Ok(result)
    }

    /// Execute given `request` given `BrokerClient`.
    pub async fn execute(&self, mut request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let mut retries = 0;
        loop {
            let mut retry_request = None;
            match &self.auth {
                Auth::None => (),
                Auth::Basic { username, password } => {
                    retry_request = Some(request.try_clone().unwrap());
                    request = request.basic_auth(username, Some(password));
                }
                Auth::Token { token } => {
                    retry_request = Some(request.try_clone().unwrap());
                    request = request.bearer_auth(token);
                }
            }
            let result = request.send().await;
            let status = match &result {
                Ok(v) => Some(v.status()),
                Err(e) => e.status(),
            };
            if let Some(StatusCode::UNAUTHORIZED) = status {
                if let Some(retry) = retry_request {
                    if retries < MAX_RETRIES {
                        retries += 1;
                        request = retry;
                        continue;
                    }
                }
            }
            return result.context(error::HttpSnafu);
        }
    }
}

/// # Utility Methods
impl BrokerClient {
    /// Returns an absolute url version of `url` using the `base_url`
    pub fn absolute_url(&self, url: impl AsRef<str>) -> Result<Url> {
        self.base_url.join(url.as_ref()).context(error::UrlSnafu)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::client::Builder;

    #[rstest]
    #[case::a_path("/api/v1/foo", "http://localhost/api/v1/foo")]
    #[case::escape_char("/help wanted", "http://localhost/help%20wanted")]
    #[case::relative_url("api/v2/bar", "http://localhost/api/v2/bar")]
    fn should_absolute_an_url(#[case] path: &str, #[case] want: &str) {
        let bc = Builder::new()
            .base_url("http://localhost")
            .unwrap()
            .build()
            .unwrap();
        let binding = bc.absolute_url(path).unwrap();
        let got = binding.as_ref();
        assert_eq!(got, want);
    }
}
