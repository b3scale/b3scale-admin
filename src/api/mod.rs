pub mod client;
pub use client::Client;
pub use client::Error as ClientError;

pub mod auth;
pub mod backends;
pub mod frontends;
pub mod status;

/// Build an API URL with the configured base URL
pub fn build_api_url(path: &str, base_url: Option<&str>) -> String {
    if let Some(base) = base_url {
        format!("{}{}", base.trim_end_matches('/'), path)
    } else {
        path.to_string()
    }
}
