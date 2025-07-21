use gloo_console;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json;
use wasm_bindgen_futures::spawn_local;
use yew::{use_effect_with_deps, use_state, UseStateHandle};

use super::auth::use_access_token;

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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
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
#[derive(Clone)]
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

        gloo_console::log!("Making API request...");
        let result = req.send().await?;
        gloo_console::log!("API response status:", result.status());
        
        if result.ok() {
            // Log the raw response text for debugging
            let response_text = result.text().await?;
            gloo_console::log!("API response body:", &response_text);
            
            // Try to decode success type from the text
            match serde_json::from_str::<T>(&response_text) {
                Ok(t) => {
                    gloo_console::log!("API request successful");
                    Ok(t)
                },
                Err(e) => {
                    gloo_console::log!("JSON parse error:", format!("{:?}", e));
                    gloo_console::log!("Raw response was:", &response_text);
                    Err(Error::Client(format!("JSON parse error: {}", e)))
                }
            }
        } else {
            // Decode error
            gloo_console::log!("API request failed with status:", result.status());
            match result.status() {
                404 => {
                    gloo_console::log!("404 Not Found error");
                    Err(Error::NotFound)
                },
                _ => {
                    // Try to decode error response
                    let err: ErrorResponse = result.json().await?;
                    gloo_console::log!("Server error:", format!("{:?}", err));
                    Err(Error::Server(err))
                }
            }
        }
    }
}

/// Provide a configured client by using the auth context
pub fn use_client() -> Client {
    let access_token = use_access_token().unwrap_or("".into());
    Client::new(&access_token)
}

/// State
#[derive(Clone, PartialEq, Debug)]
pub struct State<T: DeserializeOwned + Clone> {
    is_loading: UseStateHandle<bool>,
    error: UseStateHandle<Option<Error>>,
    result: UseStateHandle<Option<T>>,
    fetch: UseStateHandle<u32>,
}

impl<T: DeserializeOwned + Clone> State<T> {
    pub fn is_loading(&self) -> bool {
        (*self.is_loading).clone()
    }

    pub fn error(&self) -> Option<Error> {
        (*self.error).clone()
    }

    pub fn result(&self) -> Option<T> {
        (*self.result).clone()
    }

    pub fn fetch(&self) {
        let f = *self.fetch;
        self.fetch.set(f + 1);
    }
}

/// Use request returns a state object wrapping the
/// requested type
pub fn use_fetch<T: DeserializeOwned + Clone + 'static>(req: Request) -> State<T> {
    gloo_console::log!("use_fetch called");
    let client = use_client();
    let is_loading = use_state(|| false);
    let error = use_state(|| None);
    let result = use_state(|| None);
    let fetch = use_state(|| 0);

    let state = {
        let fetch = fetch.clone();
        State {
            is_loading,
            error,
            result,
            fetch,
        }
    };
    {
        let state = state.clone();
        let fetch = fetch.clone();
        use_effect_with_deps(
            move |_| {
                gloo_console::log!("use_fetch effect triggered, fetch count:", *state.fetch);
                if *state.fetch == 0 {
                    gloo_console::log!("Skipping fetch because count is 0");
                } else {
                    state.is_loading.set(true);
                    gloo_console::log!("Starting fetch...");
                    spawn_local(async move {
                        match client.fetch::<T>(req).await {
                            Ok(s) => {
                                state.is_loading.set(false);
                                state.error.set(None);
                                state.result.set(Some(s));
                            }
                            Err(error) => {
                                state.is_loading.set(false);
                                state.error.set(Some(error));
                            }
                        }
                    });
                }
                || ()
            },
            *fetch,
        );
    }
    state
}
