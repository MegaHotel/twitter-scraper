use crate::constants::{DEFAULT_AUTH_TOKEN, GENERATE_GUEST_TOKEN_URL};
use reqwest::Client;
use serde_json::Value;

pub async fn get_headers(
    auth_token_option: Option<&'static str>,
    guest_token_option: Option<&'static str>,
) -> Result<[(&'static str, &'static str); 2], Box<dyn std::error::Error>> {
    let auth_token: &'static str = auth_token_option.unwrap_or(DEFAULT_AUTH_TOKEN);
    let guest_token: &'static str;
    if guest_token_option.is_some() {
        guest_token = guest_token_option.unwrap();
    } else {
        let client: Client = Client::new();
        let response = client
            .post(GENERATE_GUEST_TOKEN_URL)
            .header("authorization", auth_token)
            .send()
            .await?
            .error_for_status()?;
        let body_data: Value = response.json::<Value>().await?;
        let boxed_guest_token: Box<str> = body_data["guest_token"]
            .as_str()
            .unwrap()
            .to_string()
            .into_boxed_str();
        // we have to force string from request body to be static str for headers map for the next request
        guest_token = Box::leak(boxed_guest_token);
    }
    let headers: [(&'static str, &'static str); 2] = [
        ("authorization", auth_token),
        ("x-guest-token", guest_token),
    ];
    Ok(headers)
}
