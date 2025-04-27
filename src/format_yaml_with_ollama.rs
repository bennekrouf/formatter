use reqwest::Client;
use std::error::Error;
use std::fs;
use tracing::{debug, error, info};

use crate::{
    extract_yaml::extract_yaml,
    load_prompt::load_prompt,
    models::{OllamaRequest, OllamaResponse},
    yaml_validator,
};

// The main function that will process the file
pub async fn format_yaml_with_ollama(
    input_file_path: &str,
    template_file_path: &str,
    system_prompt_path: &str,
    user_prompt_path: &str,
) -> Result<String, Box<dyn Error>> {
    // Read the input file
    let input_content = fs::read_to_string(input_file_path)?;
    debug!("Read input file: {}", input_file_path);

    // Read the template file
    let template_content = fs::read_to_string(template_file_path)?;
    debug!("Read template file: {}", template_file_path);

    // Load prompt templates
    let system_prompt = load_prompt(system_prompt_path)?;
    let user_prompt_template = load_prompt(user_prompt_path)?;

    // Replace placeholders in the user prompt
    let user_prompt = user_prompt_template
        .replace("{INPUT_CONTENT}", &input_content)
        .replace("{TEMPLATE_CONTENT}", &template_content);

    // Call Ollama API
    info!("Calling Ollama API");
    let client = Client::new();
    let request = OllamaRequest {
        model: "deepseek-r1:8b".to_string(),
        prompt: user_prompt,
        stream: false,
        system: system_prompt,
    };

    let resp = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await?;

    if !resp.status().is_success() {
        let error_text = resp.text().await?;
        error!("Failed to call Ollama: {}", error_text);
        return Err(format!("Ollama API error: {}", error_text).into());
    }

    let ollama_response: OllamaResponse = resp.json().await?;
    info!("Received response from Ollama");

    // Extract only the YAML part if there are any explanations
    let yaml_content = extract_yaml(&ollama_response.response);
    debug!("Extracted YAML: {}", yaml_content);

    // Validate and fix YAML indentation issues
    let fixed_yaml = yaml_validator::validate_and_fix_yaml(&yaml_content)?;
    info!("YAML validated and fixed");

    Ok(fixed_yaml)
}
