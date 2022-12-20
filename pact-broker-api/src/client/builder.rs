use std::time::Duration;

use super::{auth::Auth, error, BrokerClient, Result};
use reqwest::header::HeaderName;
use snafu::ResultExt;
use url::Url;

/// A builder struct for `BrokerClient`, allowing you to configure the client.
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let api = pact_broker_api::client::Builder::new()
///         .base_url("http://my-pact-broker")?
///         .basic_auth("username", "Pa55w0rd")
///         .build()?;
/// #    Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct Builder {
    base_url: Option<Url>,
    extra_headers: Vec<(HeaderName, String)>,
    timeout: Option<Duration>,
    auth: Auth,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Authenticate as a Basic Auth
    /// with username and password.
    pub fn basic_auth(mut self, username: &str, password: &str) -> Self {
        self.auth = Auth::Basic {
            username: username.to_owned(),
            password: password.to_owned(),
        };
        self
    }

    /// Authenticate with a JWT token.
    pub fn token(mut self, token: &str) -> Self {
        self.auth = Auth::Token {
            token: token.to_owned(),
        };
        self
    }

    /// Set the base url for `BrokerClient`.
    pub fn base_url(mut self, base_url: impl reqwest::IntoUrl) -> Result<Self> {
        self.base_url = Some(base_url.into_url().context(error::HttpSnafu)?);
        Ok(self)
    }

    /// Add an additional header to include with every request.
    pub fn add_header(mut self, key: HeaderName, value: String) -> Self {
        self.extra_headers.push((key, value));
        self
    }

    /// Configure a timeout for each request
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Create the `BrokerClient` instance.
    pub fn build(self) -> Result<BrokerClient> {
        let mut headers = reqwest::header::HeaderMap::new();

        let auth = match self.auth {
            Auth::None => Auth::None,
            Auth::Basic { username, password } => Auth::Basic { username, password },
            Auth::Token { token } => {
                headers.append(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {token}").parse().unwrap(),
                );
                Auth::None
            }
        };

        self.extra_headers.into_iter().for_each(|(key, value)| {
            headers.append(key, value.parse().unwrap());
        });

        let client = reqwest::Client::builder()
            .user_agent("pact-broker-api/rust")
            .default_headers(headers)
            .timeout(self.timeout.unwrap_or(Duration::from_millis(3000)))
            .build()
            .context(error::HttpSnafu)?;

        Ok(BrokerClient {
            client,
            base_url: self.base_url.unwrap(),
            auth,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::client::auth::Auth;

    use super::Builder;

    #[test]
    fn should_build_a_client() {
        let client = Builder::new()
            .base_url("http://localhost")
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(client.auth, Auth::None);
        assert_eq!(client.base_url.as_str(), "http://localhost/");
    }

    #[test]
    fn should_build_a_client_with_basic_auth() {
        let client = Builder::new()
            .base_url("http://localhost")
            .unwrap()
            .basic_auth("John", "secret")
            .build()
            .unwrap();
        assert_eq!(
            client.auth,
            Auth::Basic {
                username: "John".to_owned(),
                password: "secret".to_owned()
            }
        );
        assert_eq!(client.base_url.as_str(), "http://localhost/");
    }

    #[test]
    fn should_build_a_client_with_token() {
        let client = Builder::new()
            .base_url("http://localhost")
            .unwrap()
            .token("foobar123")
            .build()
            .unwrap();
        assert_eq!(client.auth, Auth::None);
        assert_eq!(client.base_url.as_str(), "http://localhost/");
    }
}
