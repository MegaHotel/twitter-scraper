use super::utils::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(deserialize_with = "str_to_datetime")]
    #[serde(serialize_with = "datetime_to_str")]
    created_at: DateTime<Utc>,
    #[serde(rename = "id_str")]
    #[serde(deserialize_with = "int_from_str_id")]
    id: u64,
    name: String,
    screen_name: String,
    description: String,
    location: String,
    statuses_count: u64,
    followers_count: u64,
    friends_count: u64,
    favourites_count: u64,
    listed_count: u64,
    media_count: u64,
    profile_image_url_https: String,
    verified: bool,
    #[serde(rename = "profile_image_url_https")]
    #[serde(default)]
    image: Option<String>,
}
