#[derive(Default, Debug, Clone, PartialEq)]

/// Authentication used in `BrockerClient`
pub enum Auth {
    /// No Authentication.
    #[default]
    None,
    /// Basic Auth HTTP Header to be set to provide authentication
    Basic { username: String, password: String },
    /// Set the HTTP Header authorization
    Token { token: String },
}
