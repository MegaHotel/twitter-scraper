use super::utils::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    #[serde(deserialize_with = "str_to_datetime")]
    #[serde(serialize_with = "datetime_to_str")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "id_str")]
    #[serde(deserialize_with = "int_from_str_id")]
    pub id: u64,
    #[serde(rename = "full_text")]
    pub text: String,
    #[serde(default)]
    pub tweet_url: Option<String>,
    #[serde(rename = "entities")]
    #[serde(deserialize_with = "from_entities")]
    pub entities: Entities,
    #[serde(rename = "in_reply_to_status_id_str")]
    #[serde(deserialize_with = "int_opt_from_str_id")]
    pub in_reply_to_tweet_id: Option<u64>,
    #[serde(rename = "in_reply_to_user_id_str")]
    #[serde(deserialize_with = "int_opt_from_str_id")]
    pub in_reply_to_user_id: Option<u64>,
    #[serde(rename = "source")]
    #[serde(deserialize_with = "platform_enum_from_source")]
    pub platform: Platform,
    pub favorite_count: usize,
    pub quote_count: usize,
    pub reply_count: usize,
    pub retweet_count: usize,
    #[serde(rename = "lang")]
    pub language: String,
    pub card: Option<Card>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entities {
    pub hashtags: Vec<String>,
    pub media: Vec<Media>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Media {
    pub id: u64,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub url: String,
    #[serde(rename = "binding_values")]
    #[serde(deserialize_with = "image_url_from_card")]
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Platform {
    Web,
    Android,
    IPhone,
    Unknown,
}
