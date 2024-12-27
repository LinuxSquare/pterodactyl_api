//! API for endpoints under `api/application/locations`

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{http::EmptyBody, structs::{PteroList, PteroObject}};

use super::Application;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// A location of a node
pub struct Location {
  /// The ID of this location
  pub id: u32,

  /// The short code of this location
  pub short: String,

  /// The long description of this location
  pub long: String,

  /// The creation timestamp of this location
  #[serde(deserialize_with = "crate::structs::iso_time")]
  pub created_at: OffsetDateTime,

  /// The update timestamp of this location
  #[serde(deserialize_with = "crate::structs::iso_time")]
  pub updated_at: OffsetDateTime
}

impl Application<'_> {
  /// Retrieves all locations
  pub async fn list_locations(&self) -> crate::Result<Vec<Location>> {
    self.client
      .request::<PteroList<Location>>(Method::GET, &format!("locations"))
      .await
      .map(|locations|locations.data)
  }

  /// Retrieves the specified location
  pub async fn get_location(&self, id: u32) -> crate::Result<Location> {
    self.client
      .request::<PteroObject<Location>>(Method::GET, &format!("locations/{}", id))
      .await
      .map(|location|location.attributes)
  }

  /// Adds a location with the given short code
  pub async fn add_location(
    &self,
    short: impl Into<String>,
    long: impl Into<String>
  ) -> crate::Result<Location> {
    #[derive(Serialize)]
    struct AddLocationBody {
      short: String,
      long: String
    }
    self.client
      .request_with_body::<PteroObject<Location>, _>(
        Method::POST, 
        &format!("locations"),
        &AddLocationBody {
          short: short.into(),
          long: long.into()
        }
      )
      .await
      .map(|location|location.attributes)
  }

  /// Updates the location by its id
  pub async fn update_location(
    &self,
    id: u32,
    short: impl Into<String>,
    long: impl Into<String>
  ) -> crate::Result<Location> {
    #[derive(Serialize)]
    struct UpdateLocationBody {
      short: String,
      long: String
    }
    self.client
      .request_with_body::<PteroObject<Location>, _>(
        Method::PATCH, 
        &format!("locations/{}", id), 
        &UpdateLocationBody {
          short: short.into(),
          long: long.into()
        }
      )
      .await
      .map(|location|location.attributes)
  }

  /// Removes a location from the panel
  pub async fn delete_location(&self, id: u32) -> crate::Result<()> {
    self.client
      .request::<EmptyBody>(Method::DELETE, &format!("locations/{}", id))
      .await?;
    Ok(())
  }
}
