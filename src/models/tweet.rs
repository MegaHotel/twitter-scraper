use super::date_format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    #[serde(with = "date_format")]
    created_at: DateTime<Utc>,
    #[serde(rename = "id_str")]
    #[serde(deserialize_with = "int_from_str_id")]
    id: u64,
    #[serde(rename = "full_text")]
    text: String,
    #[serde(default)]
    tweet_url: Option<String>,
    #[serde(rename = "entities")]
    #[serde(deserialize_with = "from_entities")]
    entities: Entities,
    #[serde(rename = "in_reply_to_status_id_str")]
    #[serde(deserialize_with = "int_opt_from_str_id")]
    in_reply_to_tweet_id: Option<u64>,
    #[serde(rename = "in_reply_to_user_id_str")]
    #[serde(deserialize_with = "int_opt_from_str_id")]
    in_reply_to_user_id: Option<u64>,
    #[serde(rename = "source")]
    #[serde(deserialize_with = "platform_enum_from_source")]
    platform: Platform,
    favorite_count: usize,
    quote_count: usize,
    reply_count: usize,
    retweet_count: usize,
    #[serde(rename = "lang")]
    language: String,
    card: Option<Card>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Entities {
    hashtags: Vec<String>,
    media: Vec<Media>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Media {
    id: u64,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Card {
    url: String,
    #[serde(rename = "binding_values")]
    #[serde(deserialize_with = "image_url_from_card")]
    image_url: String,
}
#[derive(Debug, Deserialize, Serialize)]
enum Platform {
    Web,
    Android,
    IPhone,
    Unknown,
}

fn int_from_str_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer)?;
    string.parse().map_err(serde::de::Error::custom)
}

fn int_opt_from_str_id<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let string: Value = Deserialize::deserialize(deserializer)?;
    match string {
        Value::Null => Ok(None),
        _ => {
            let id: u64 = string.as_str().unwrap().parse().unwrap();
            Ok(Some(id))
        }
    }
}

fn from_entities<'de, D>(deserializer: D) -> Result<Entities, D::Error>
where
    D: Deserializer<'de>,
{
    let entities: Value = Deserialize::deserialize(deserializer)?;
    let hashtags_array = entities["hashtags"].clone();
    let hashtags: Vec<String> = hashtags_array
        .as_array()
        .unwrap()
        .iter()
        .map(|hashtag_object| hashtag_object["text"].as_str().unwrap().to_string())
        .collect();
    let media_array: Option<&Vec<Value>> = entities["media"].as_array();
    if let Some(media) = media_array {
        let media_vec: Vec<Media> = media
            .iter()
            .map(|media_object| Media {
                id: media_object["id_str"].as_str().unwrap().parse().unwrap(),
                url: media_object["media_url_https"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            })
            .collect();
        Ok(Entities {
            hashtags,
            media: media_vec,
        })
    } else {
        Ok(Entities {
            hashtags,
            media: vec![],
        })
    }
}

fn platform_enum_from_source<'de, D>(deserializer: D) -> Result<Platform, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer)?;
    match string.as_str() {
        "<a href=\"https://mobile.twitter.com\" rel=\"nofollow\">Twitter Web App</a>" => Ok(Platform::Web),
        "<a href=\"http://twitter.com/download/android\" rel=\"nofollow\">Twitter for Android</a>" => Ok(Platform::Android),
        "<a href=\"http://twitter.com/download/iphone\" rel=\"nofollow\">Twitter for iPhone</a>" => Ok(Platform::IPhone),
        _ => Ok(Platform::Unknown),
    }
}

fn image_url_from_card<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let card_object: Value = Deserialize::deserialize(deserializer)?;
    let media_url = card_object["photo_image_full_size"]["image_value"]["url"]
        .as_str()
        .unwrap()
        .to_string();
    Ok(media_url)
}
