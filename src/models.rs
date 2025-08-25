use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CohereRequest {
    pub model: String,
    pub message: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f64>,
    pub chat_history: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct CohereResponse {
    pub text: String,
}
