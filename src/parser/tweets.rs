use crate::models::Tweet;
use serde_json::Value;
use std::collections::HashMap;

type TweetsMap = HashMap<String, Tweet>;

pub fn get_tweets(body_data: &Value) -> Vec<Tweet> {
    let tweets_map: TweetsMap =
        serde_json::from_value(body_data["globalObjects"]["tweets"].clone()).unwrap();
    tweets_map.into_values().collect()
}
