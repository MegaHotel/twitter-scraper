use reqwest::{Client, Response};
mod models;
mod parser;
mod request_builder;
use serde_json::Value;

use crate::parser::{get_next_cursor, get_tweets};

const BASE_URL: &str = "https://twitter.com/i/api/2/search/adaptive.json?";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let headers_tuples = vec![("authorization", ""), ("x-guest-token", "")];
    let request_config = request_builder::build_request_config(headers_tuples, "#Trump", None);
    let client: Client = Client::new();
    let response: Response = client
        .get(BASE_URL)
        .query(&request_config.path_query)
        .headers(request_config.headers)
        .send()
        .await?
        .error_for_status()
        .unwrap();
    let body_data: Value = response.json::<Value>().await?;
    let _cursor: String = get_next_cursor(&body_data).await?;
    let tweets = get_tweets(&body_data);
    println!("{:?}", tweets);
    Ok(())
}
