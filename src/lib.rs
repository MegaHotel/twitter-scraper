mod parser;
use models::{Tweet, User};
use request_builder::RequestConfig;
use reqwest::{Client, Response};
mod models;
mod request_builder;
use parser::{get_next_cursor, get_tweets, get_users};
use serde_json::Value;
const BASE_URL: &str = "https://twitter.com/i/api/2/search/adaptive.json?";

#[derive(Debug)]
pub struct TwitterResults {
    pub cursor: String,
    pub tweets: Option<Vec<Tweet>>,
    pub users: Option<Vec<User>>,
}

#[tokio::main]
pub async fn run(
    auth_token: &'static str,
    guest_token: &'static str,
) -> Result<TwitterResults, Box<dyn std::error::Error>> {
    let headers_tuples: [(&str, &str); 2] = [
        ("authorization", auth_token),
        ("x-guest-token", guest_token),
    ];
    let request_config: RequestConfig =
        request_builder::build_request_config(&headers_tuples, "#Trump", None);
    let client: Client = Client::new();
    let response: Response = client
        .get(BASE_URL)
        .query(&request_config.path_query)
        .headers(request_config.headers)
        .send()
        .await?
        .error_for_status()?;
    let body_data: Value = response.json::<Value>().await?;
    let cursor: String = get_next_cursor(&body_data)?;
    let tweets: Vec<Tweet> = get_tweets(&body_data);
    let users: Vec<User> = get_users(&body_data);
    let twitter_results = TwitterResults {
        cursor,
        tweets: Some(tweets),
        users: Some(users),
    };
    Ok(twitter_results)
}
