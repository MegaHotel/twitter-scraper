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
pub fn get_next_cursor(
    body_data: &Value,
    curr_cursor: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    if curr_cursor.is_some() {
        get_default_next_cursor(body_data)
    } else {
        get_first_next_cursor(body_data)
    }
}

fn get_first_next_cursor(body_data: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let cursor_entry: &Value = body_data["timeline"]["instructions"][0]["addEntries"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .find(|entry| find_bottom_cursor_entry(entry))
        .unwrap();
    Ok(get_value_from_cursor_entry(cursor_entry))
}

fn get_default_next_cursor(body_data: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let cursor_entry: Value = body_data["timeline"]["instructions"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .find_map(|object| {
            if let Some(replace_entry_object) = object["replaceEntry"].as_object() {
                let entry_object = replace_entry_object["entry"].clone();
                if find_bottom_cursor_entry(&entry_object) {
                    Some(entry_object)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();
    Ok(get_value_from_cursor_entry(&cursor_entry))
}

fn find_bottom_cursor_entry(entry: &Value) -> bool {
    if let Some(entry_id) = entry["entryId"].as_str() {
        entry_id == "sq-cursor-bottom"
    } else {
        false
    }
}

fn get_value_from_cursor_entry(cursor_entry: &Value) -> String {
    cursor_entry["content"]["operation"]["cursor"]["value"]
        .as_str()
        .unwrap()
        .to_string()
}
