use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serializer};

// "Mon Aug 01 02:16:18 +0000 2022"
const DE_FORMAT: &str = "%a %b %d %H:%M:%S %z %Y";
const SE_FORMAT: &str = "%+";

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, DE_FORMAT)
        .map_err(serde::de::Error::custom)
}

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(SE_FORMAT));
    serializer.serialize_str(&s)
}
