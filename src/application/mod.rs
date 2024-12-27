//! Pterodactyl Application API implementation, for all endpoints under `api/application`

use crate::client::Client;

pub mod users;
pub mod locations;
pub mod nodes;

/// represents a Pterodactyl application, with which requests specific to the host can be made
#[derive(Debug)]
pub struct Application<'a> {
  pub(crate) client: &'a Client
}
