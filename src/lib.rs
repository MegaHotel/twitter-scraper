mod constants;
mod headers;
mod models;
mod parser;
mod request_builder;
use constants::TWITTER_BASE_URL;
use headers::get_headers;
use models::{Tweet, User};
use parser::{get_next_cursor, get_tweets, get_users};
use request_builder::RequestConfig;
use reqwest::{Client, Response};
use serde_json::Value;

#[derive(Debug)]
pub struct TwitterResults {
    pub cursor: String,
    pub tweets: Option<Vec<Tweet>>,
    pub users: Option<Vec<User>>,
}

#[tokio::main]
pub async fn run(
    query: &str,
    auth_token_option: Option<&'static str>,
    guest_token_option: Option<&'static str>,
    cursor: Option<&'static str>,
) -> Result<TwitterResults, Box<dyn std::error::Error>> {
    let headers_tuples: [(&'static str, &'static str); 2] =
        get_headers(auth_token_option, guest_token_option).await?;
    let request_config: RequestConfig =
        request_builder::build_request_config(&headers_tuples, query, cursor);
    let client: Client = Client::new();
    let response: Response = client
        .get(TWITTER_BASE_URL)
        .query(&request_config.path_query)
        .headers(request_config.headers)
        .send()
        .await?
        .error_for_status()?;
    let body_data: Value = response.json::<Value>().await?;
    let next_cursor: String = get_next_cursor(&body_data, cursor)?;
    let tweets: Vec<Tweet> = get_tweets(&body_data);
    let users: Vec<User> = get_users(&body_data);
    let twitter_results = TwitterResults {
        cursor: next_cursor,
        tweets: Some(tweets),
        users: Some(users),
    };
    Ok(twitter_results)
}
