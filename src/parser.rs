use crate::models::Tweet;
use crate::models::User;
use serde_json::Value;
use std::collections::HashMap;

type TweetsMap = HashMap<String, Tweet>;
type UsersMap = HashMap<String, User>;

pub fn get_tweets(body_data: &Value) -> Vec<Tweet> {
    let tweets_map: TweetsMap =
        serde_json::from_value(body_data["globalObjects"]["tweets"].clone()).unwrap();
    tweets_map.into_values().collect()
}
pub fn get_users(body_data: &Value) -> Vec<User> {
    let users_map: UsersMap =
        serde_json::from_value(body_data["globalObjects"]["users"].clone()).unwrap();
    users_map.into_values().collect()
}
pub fn get_next_cursor(body_data: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let cursor_entry: &Value = body_data["timeline"]["instructions"][0]["addEntries"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .find(|entry| {
            if let Some(entry_id) = entry["entryId"].as_str() {
                entry_id == "sq-cursor-bottom"
            } else {
                false
            }
        })
        .unwrap();
    let cursor = cursor_entry["content"]["operation"]["cursor"]["value"]
        .as_str()
        .unwrap()
        .to_string();
    Ok(cursor)
}
