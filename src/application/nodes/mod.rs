//! API for endpoints under `api/application/nodes`

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{http::EmptyBody, structs::{PteroList, PteroObject}};

use super::Application;

pub mod allocations;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// A node which contains servers
pub struct Node {
  /// The ID of this node
  pub id: u32,

  /// The unique identifier of this node
  pub uuid: Uuid,

  /// Is the node public?
  pub public: bool,

  /// The name of this node
  pub name: String,

  /// The description of this node
  pub description: String,

  /// The location id of this node
  pub location_id: u32,

  /// The full qualified domain name (fqdn) of this node
  pub fqdn: String,

  /// The scheme (http/https) of this node
  pub scheme: String,

  /// Is this node behind a proxy?
  pub behind_proxy: bool,

  /// Is this node in maintenance mode?
  pub maintenance_mode: bool,

  /// The allocated memory (in bytes) of this node
  pub memory: u32,

  /// The maximum memory overallocation (in bytes) of this node
  pub memory_overallocate: i32,

  /// The disk-size (in bytes) of this node
  #[serde(rename = "disk")]
  pub disksize: u32,

  /// The maximum disk overallocation (in bytes) of this node
  pub disk_overallocate: i32,

  /// The maximum upload size (in bytes) of this node
  pub upload_size: u32,

  /// The listen-port of the wings daemon for this node
  pub daemon_listen: u32,

  /// The daemon sftp port for this node
  pub daemon_sftp: u32,

  /// The daemon base path for this node
  pub daemon_base: String,

  /// The creation timestamp of this node
  #[serde(deserialize_with = "crate::structs::iso_time")]
  pub created_at: OffsetDateTime,

  /// The update timestamp of this node
  #[serde(deserialize_with = "crate::structs::iso_time")]
  pub updated_at: OffsetDateTime
}

impl Application {
  /// Retrieves all nodes
  pub async fn list_nodes(&self) -> crate::Result<Vec<Node>> {
    self.request::<PteroList<Node>>(Method::GET, &format!("nodes"))
      .await
      .map(|nodes|nodes.data)
  }

  /// Retrieves the specified node
  pub async fn get_node(&self, id: u32) -> crate::Result<Node> {
    self.request::<PteroObject<Node>>(Method::GET, &format!("nodes/{}", id))
      .await
      .map(|node|node.attributes)
  }

  // /// Retrieves the node configuration
  //pub async fn get_node_configuration(&self, id: u32) -> crate::Result<No>

  /// Create a new node
  pub async fn create_node(
    &self,
    name: impl Into<String>,
    location_id: impl Into<u32>,
    fqdn: impl Into<String>,
    scheme: impl Into<String>,
    memory: impl Into<u32>,
    memory_overallocate: impl Into<u32>,
    disk: impl Into<u32>,
    disk_overallocate: impl Into<u32>,
    upload_size: impl Into<u32>,
    daemon_sftp: impl Into<u32>,
    daemon_listen: impl Into<u32>
  ) -> crate::Result<Node> {
    #[derive(Serialize)]
    struct AddNodeBody {
      name: String,
      location_id: u32,
      fqdn: String,
      scheme: String,
      memory: u32,
      memory_overallocate: u32,
      disk: u32,
      disk_overallocate: u32,
      upload_size: u32,
      daemon_sftp: u32,
      daemon_listen: u32
    }
    self.request_with_body::<PteroObject<Node>, _>(
        Method::POST, 
        &format!("nodes"), 
        &AddNodeBody {
          name: name.into(),
          location_id: location_id.into(),
          fqdn: fqdn.into(),
          scheme: scheme.into(),
          memory: memory.into(),
          memory_overallocate: memory_overallocate.into(),
          disk: disk.into(),
          disk_overallocate: disk_overallocate.into(),
          upload_size: upload_size.into(),
          daemon_sftp: daemon_sftp.into(),
          daemon_listen: daemon_listen.into()
        }
      )
      .await
      .map(|node|node.attributes)
  }

  /// Updates the node by its id
  pub async fn update_node(
    &self,
    id: u32,
    name: impl Into<String>,
    description: impl Into<String>,
    location_id: impl Into<u32>,
    fqdn: impl Into<String>,
    scheme: impl Into<String>,
    behind_proxy: impl Into<bool>,
    maintenance_mode: impl Into<bool>,
    memory: impl Into<u32>,
    memory_overallocate: impl Into<u32>,
    disk: impl Into<u32>,
    disk_overallocate: impl Into<u32>,
    upload_size: impl Into<u32>,
    daemon_sftp: impl Into<u32>,
    daemon_listen: impl Into<u32>
  ) -> crate::Result<Node> {
    #[derive(Serialize)]
    struct UpdateNodeBody {
      name: String,
      description: String,
      location_id: u32,
      fqdn: String,
      scheme: String,
      behind_proxy: bool,
      maintenance_mode: bool,
      memory: u32,
      memory_overallocate: u32,
      disk: u32,
      disk_overallocate: u32,
      upload_size: u32,
      daemon_sftp: u32,
      daemon_listen: u32      
    }
    self.request_with_body::<PteroObject<Node>, _>(
        Method::PATCH,
        &format!("nodes/{}", id),
        &UpdateNodeBody {
          name: name.into(),
          description: description.into(),
          location_id: location_id.into(),
          fqdn: fqdn.into(),
          scheme: scheme.into(),
          behind_proxy: behind_proxy.into(),
          maintenance_mode: maintenance_mode.into(),
          memory: memory.into(),
          memory_overallocate: memory_overallocate.into(),
          disk: disk.into(),
          disk_overallocate: disk_overallocate.into(),
          upload_size: upload_size.into(),
          daemon_sftp: daemon_sftp.into(),
          daemon_listen: daemon_listen.into()
        }
      )
      .await
      .map(|node|node.attributes)
  }

  /// Removes a node
  pub async fn delete_node(&self, id: u32) -> crate::Result<()> {
    self.request::<EmptyBody>(Method::DELETE, &format!("nodes/{}", id))
      .await?;
    Ok(())
  }

}
