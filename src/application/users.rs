//! API for endpoints under `api/application/users`

use reqwest::Method;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{http::EmptyBody, structs::{PteroList, PteroObject}};

use super::Application;

#[derive(Debug, Deserialize)]
#[non_exhaustive]
/// A user for the panel login
pub struct User {
  /// The ID of this user
  pub id: u32,
  
  /// The external id of this user
  pub external_id: String,
  
  /// The unique identifier of this user
  pub uuid: Uuid,
  
  /// The username of this user
  pub username: String,

  /// The email of this user
  pub email: String,

  /// The user's first name
  pub first_name: String,

  /// The user's last name
  pub last_name: String,

  /// The user's language
  pub language: String,

  /// Has the user admin privileges?
  pub root_admin: bool,

  /// Two factor enabled?
  #[serde(rename = "2fa")]
  pub two_factor_enabled: bool,

  /// The creation timestamp of this user
  #[serde(deserialize_with = "crate::structs::iso_time")]
  pub created_at: OffsetDateTime,

  /// The update timestamp of this user
  #[serde(deserialize_with = "crate::structs::iso_time")]
  pub updated_at: OffsetDateTime
}

impl Application {
  /// Retrieves all users
  pub async fn list_users(&self) -> crate::Result<Vec<User>> {
    self.request::<PteroList<User>>(Method::GET, &format!("users"))
      .await
      .map(|users| users.data)
  }

  /// Retrieves the specified user
  pub async fn get_user(&self, id: u32) -> crate::Result<User> {
    self.request::<PteroObject<User>>(Method::GET, &format!("users/{}", id))
      .await
      .map(|user|user.attributes)
  }
  
  /// Retrieves the specified user by its external ID
  pub async fn get_user_external(&self, external_id: String) -> crate::Result<User> {
    self.request::<PteroObject<User>>(Method::GET, &format!("users/{}", external_id))
    .await
    .map(|user|user.attributes)
  }

  /// Adds a user with the given email, username, first_name & last_name to the panel
  pub async fn add_user(
    &self,
    email: impl Into<String>,
    username: impl Into<String>,
    first_name: impl Into<String>,
    last_name: impl Into<String>
  ) -> crate::Result<User> {
    #[derive(Serialize)]
    struct AddUserBody {
      email: String,
      username: String,
      first_name: String,
      last_name: String
    }
    self.request_with_body::<PteroObject<User>, _>(
        Method::POST, 
        &format!("users"), 
        &AddUserBody {
          email: email.into(),
          username: username.into(),
          first_name: first_name.into(),
          last_name: last_name.into()
        },
      )
      .await
      .map(|user|user.attributes)
  }

  /// Updates the user by its id
  pub async fn update_user(
    &self,
    id: u32,
    email: impl Into<String>,
    username: impl Into<String>,
    first_name: impl Into<String>,
    last_name: impl Into<String>,
    language: impl Into<String>,
    password: impl Into<String>
  ) -> crate::Result<User> {
    #[derive(Serialize)]
    struct UpdateUserBody {
      email: String,
      username: String,
      first_name: String,
      last_name: String,
      language: String,
      password: String
    }
    self.request_with_body::<PteroObject<User>, _>(
        Method::PATCH, 
        &format!("users/{}", id), 
        &UpdateUserBody {
          email: email.into(),
          username: username.into(),
          first_name: first_name.into(),
          last_name: last_name.into(),
          language: language.into(),
          password: password.into()
        }
      )
      .await
      .map(|user|user.attributes)
  }

  /// Removes a user from the panel
  pub async fn delete_user(&self, id: u32) -> crate::Result<()> {
    self.request::<EmptyBody>(Method::DELETE, &format!("users/{}", id))
      .await?;
    Ok(())
  }
}
