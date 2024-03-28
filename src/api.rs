use reqwest::{self, StatusCode};
use serde::Serialize;
use serde_json::Value;
use std::str;
use tokio;
pub struct OpenAI {
    api_key: String,
    current_request_body: TextCompletionRequestBody,
}

#[derive(Serialize)]
pub struct ConversationMessage {
    role: String,
    content: String,
}

pub type Conversation = Vec<ConversationMessage>;

#[derive(Serialize)]
struct TextCompletionRequestBody {
    model: String,
    messages: Conversation,
}

impl OpenAI {
    const URL_TEXT_COMPLETION: &str = "https://api.openai.com/v1/chat/completions";

    pub fn new(key: &str, model: &str, system_prompt: &str) -> Self {
        let conversation: Conversation = vec![ConversationMessage {
            role: String::from("system"),
            content: String::from(system_prompt)
        }];
        let body = TextCompletionRequestBody {
            model: String::from(model),
            messages: conversation,
        };
        return Self {
            api_key: String::from(key),
            current_request_body: body,
        };
    }

    // #[tokio::main]
    // pub async fn greet(self) -> String {
    //     let body = TextCompletionRequestBody {
    //         model: "gpt-3.5-turbo",
    //         messages: vec![
    //             ConversationMessage {
    //                 role: "system",
    //                 content: "You are an evilly and aggressively helpful assistant.",
    //             },
    //             ConversationMessage {
    //                 role: "user",
    //                 content: "How are you",
    //             },
    //         ],
    //     };
    //     let value = self
    //         .api_builder
    //         .json(&body)
    //         .send()
    //         .await
    //         .expect("request failed!");
    //     match value.status() {
    //         StatusCode::OK => {
    //             let v = value.text().await.expect("invalid text");
    //             let json =
    //                 serde_json::from_str::<serde_json::Value>(&v).expect("cannot decode JSON");
    //             return format!(
    //                 "{}",
    //                 json["choices"][0]["message"]["content"]
    //                     .as_str()
    //                     .expect("failed to convert to string")
    //             );
    //         }
    //         _ => {
    //             return format!("bruh: {}", value.status());
    //         }
    //     }
    // }
    fn push_message(&mut self, role: &str, content: &str) {
        self.current_request_body
            .messages
            .push(ConversationMessage {
                role: String::from(role),
                content: String:: from(content),
            });
    }

    #[tokio::main]
    pub async fn chat(&mut self, new_message: &str) -> String {
        self.push_message("user", new_message);
        let value = reqwest::Client::new()
            .post(OpenAI::URL_TEXT_COMPLETION)
            .bearer_auth(&self.api_key)
            .json(&self.current_request_body)
            .send()
            .await
            .expect("request failed!");
        if value.status() == StatusCode::OK {
            let v = value.text().await.expect("invalid text");
            let json = serde_json::from_str::<Value>(&v).expect("");
            let message = format!(
                "{}",
                json["choices"][0]["message"]["content"]
                    .as_str()
                    .expect("failed to convert to string")
            );
            self.push_message("assistant", &message);
            return message;
        } else {
            return value.status().to_string();
        }
    }
}
