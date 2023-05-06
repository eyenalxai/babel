use crate::chat_gpt::ChatGPTMessage;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Response};
use std::collections::HashMap;
use std::error::Error;

pub async fn send_openai_request(
    client: &Client,
    url: &str,
    headers: HashMap<String, String>,
    messages: Vec<ChatGPTMessage>,
) -> Result<Response, Box<dyn Error>> {
    let mut header_map = HeaderMap::new();
    for (key, value) in headers {
        if let Ok(header_name) = key.parse::<HeaderName>() {
            header_map.insert(header_name, HeaderValue::from_str(&value)?);
        }
    }

    let body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": messages,
    });

    let response = client
        .post(url)
        .headers(header_map)
        .json(&body)
        .send()
        .await?;

    Ok(response)
}
