use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serializer};
use serde_json::Value;

use super::tweet::{Entities, Media, Platform};

// "Mon Aug 01 02:16:18 +0000 2022"
const DE_FORMAT: &str = "%a %b %d %H:%M:%S %z %Y";
const SE_FORMAT: &str = "%+";

pub fn str_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, DE_FORMAT)
        .map_err(serde::de::Error::custom)
}

pub fn datetime_to_str<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(SE_FORMAT));
    serializer.serialize_str(&s)
}

pub fn int_from_str_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer)?;
    string.parse().map_err(serde::de::Error::custom)
}

pub fn int_opt_from_str_id<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
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

pub fn from_entities<'de, D>(deserializer: D) -> Result<Entities, D::Error>
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

pub fn platform_enum_from_source<'de, D>(deserializer: D) -> Result<Platform, D::Error>
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

pub fn image_url_from_card<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let card_object: Value = Deserialize::deserialize(deserializer)?;
    let media_url: Option<&str> =
        card_object["photo_image_full_size"]["image_value"]["url"].as_str();
    let media_url_string: Option<String> = media_url.map(|url| url.to_string());
    Ok(media_url_string)
}
