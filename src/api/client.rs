use serde::{de::DeserializeOwned, Deserialize};

/// Reexport request for convenience
pub use gloo_net::http::Request;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct ErrorResponse {
    pub message: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotFound,
    ValidationFailed(String),
    Client(String),
    Server(ErrorResponse),
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Self::NotFound => "404 not found".into(),
            Self::ValidationFailed(msg) => msg.clone(),
            Self::Client(msg) => msg.clone(),
            Self::Server(res) => res.message.clone(),
        }
    }
}

impl From<gloo_net::Error> for Error {
    fn from(err: gloo_net::Error) -> Self {
        Self::Client(err.to_string())
    }
}

/// Client Result
pub type Result<T> = std::result::Result<T, Error>;

/// A client with an access token
pub struct Client(String);

impl Client {
    /// Create new client with token
    pub fn new(token: &str) -> Self {
        Self(token.to_owned())
    }

    /// Fetch performs the request and decodes the result
    pub async fn fetch<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let Self(auth_token) = self;
        let bearer = format!("Bearer {}", auth_token);
        let req = req.header("Authorization", &bearer);

        let result = req.send().await?;
        if result.ok() {
            // Try to decode success type
            let t: T = result.json().await?;
            Ok(t)
        } else {
            // Decode error
            match result.status() {
                404 => Err(Error::NotFound),
                _ => {
                    // Try to decode error response
                    let err: ErrorResponse = result.json().await?;
                    Err(Error::Server(err))
                }
            }
        }
    }
}
