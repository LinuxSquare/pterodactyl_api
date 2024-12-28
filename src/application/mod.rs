//! Pterodactyl Application API implementation, for all endpoints under `api/application`

use std::sync::RwLock;

use crate::client::RateLimits;

pub mod users;
pub mod locations;
pub mod nodes;

/// represents a Pterodactyl application, with which requests specific to the host can be made
#[derive(Debug)]
pub struct Application {
  pub(crate) url: String,
  pub(crate) client: reqwest::Client,
  pub(crate) api_key: String,
  pub(crate) rate_limits: RwLock<Option<RateLimits>>
}

/// A builder for an application
#[derive(Debug)]
pub struct ApplicationBuilder {
  url: String,
  client: Option<reqwest::Client>,
  api_key: String
}

impl ApplicationBuilder {
  /// Creates a new application builder, conntecting to the given URL where a Pterodactyl server is
  /// hosted, using the given API key for authentication
  pub fn new(url: impl Into<String>, api_key: impl Into<String>) -> Self {
    let mut url = url.into();
    if !url.ends_with('/') {
      url.push('/');
    }
    url.push_str("api/application/");
    Self {
      url,
      client: None,
      api_key: api_key.into()
    }
  }

  /// Uses the specified [`reqwest::Client`] for requests instead of making a default one
  pub fn with_client(self, client: reqwest::Client) -> Self {
    Self {
      client: Some(client),
      ..self
    }
  }

  /// Builds an application
  pub fn build(self) -> Application {
    Application {
      url: self.url,
      client: self.client.unwrap_or_default(),
      api_key: self.api_key,
      rate_limits: RwLock::new(None)
    }
  }
}
