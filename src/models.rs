use serde::{Deserialize, Serialize};

// Define the Ollama request structure
#[derive(Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    pub system: String,
}

// Define the Ollama response structure
#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
}
