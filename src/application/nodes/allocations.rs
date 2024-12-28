//! API for endpoints under `api/application/nodes/{node}/allocations`

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::application::Application;

use super::{EmptyBody, PteroList};

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// A allocation of a node
pub struct Allocation {
  /// The id of this allocation
  pub id: u32,

  /// The ip of this allocation
  pub ip: String,

  /// The optional alias of this allocation
  pub alias: Option<String>,

  /// The port of this allocation
  pub port: u32,

  /// The optional notes of this allocation
  pub notes: Option<String>,

  /// Has this allocation been assigned?
  pub assigned: bool
}

impl Application {
  /// Retrieves all allocations
  pub async fn list_allocations(&self, node_id: u32) -> crate::Result<Vec<Allocation>> {
    self.request::<PteroList<Allocation>>(Method::GET, &format!("nodes/{}/allocations", node_id))
    .await
    .map(|allocations|allocations.data)
  }

  /// Create allocations
  pub async fn create_allocations(
    &self,
    node_id: u32,
    ip: impl Into<String>,
    ports: impl Into<Vec<String>>
  ) -> crate::Result<()> {
    #[derive(Serialize)]
    struct AddAllocationsBody {
      ip: String,
      ports: Vec<String>
    }
    self.request_with_body::<EmptyBody, _>(
      Method::POST,
      &format!("nodes/{}/allocations", node_id),
      &AddAllocationsBody {
        ip: ip.into(),
        ports: ports.into()
      }
    ).await?;
    Ok(())
  }

  /// Delete allocation
  pub async fn delete_allocation(&self, node_id: u32, allocation_id: u32) -> crate::Result<()> {
    self.request::<EmptyBody>(Method::DELETE, &format!("nodes/{0}/allocations/{1}", node_id, allocation_id))
    .await?;
    Ok(())
  }
}
