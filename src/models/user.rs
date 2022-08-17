use super::utils::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(deserialize_with = "str_to_datetime")]
    #[serde(serialize_with = "datetime_to_str")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "id_str")]
    #[serde(deserialize_with = "int_from_str_id")]
    pub id: u64,
    pub name: String,
    pub screen_name: String,
    pub description: String,
    pub location: String,
    pub statuses_count: u64,
    pub followers_count: u64,
    pub friends_count: u64,
    pub favourites_count: u64,
    pub listed_count: u64,
    pub media_count: u64,
    pub profile_image_url_https: String,
    pub verified: bool,
    #[serde(rename = "profile_image_url_https")]
    #[serde(default)]
    pub image: Option<String>,
}
