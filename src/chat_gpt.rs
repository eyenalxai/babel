use crate::request::send_openai_request;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatGPTMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub message: ChatGPTMessage,
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletion {
    pub choices: Vec<Choice>,
}
fn first_choice(chat_completion: ChatCompletion) -> ChatGPTMessage {
    chat_completion
        .choices
        .into_iter()
        .min_by_key(|choice| choice.index)
        .expect("No choices returned from OpenAI")
        .message
}

fn get_openai_token() -> String {
    let openai_token =
        std::env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN environment variable not set");

    if !openai_token.starts_with("sk-") {
        panic!("OPENAI_TOKEN must start with sk-");
    }

    openai_token
}

pub type ChatGPTMessageFuture =
    Pin<Box<dyn Future<Output = Result<ChatGPTMessage, Box<dyn Error>>> + Send + 'static>>;
pub type ChatGPTMessageHandler = dyn Fn(Vec<ChatGPTMessage>) -> ChatGPTMessageFuture + Send + Sync;

pub fn chat_gpt_wrapper(client: &'static Client) -> Box<ChatGPTMessageHandler> {
    let openai_token = get_openai_token();
    let openai_chat_api_url = "https://api.openai.com/v1/chat/completions";

    let headers = {
        let mut map = HashMap::new();
        map.insert("Content-Type".to_string(), "application/json".to_string());
        map.insert(
            "Authorization".to_string(),
            format!("Bearer {}", openai_token),
        );
        map
    };

    let closure: Box<ChatGPTMessageHandler> = Box::new(move |messages: Vec<ChatGPTMessage>| {
        let openai_chat_api_url = openai_chat_api_url.to_owned();
        let headers = headers.clone();
        Box::pin(async move {
            let response =
                send_openai_request(client, &openai_chat_api_url, headers, messages).await?;

            let response_json = response.json().await?;

            let result = first_choice(response_json);

            Ok(result)
        })
    });

    closure
}
