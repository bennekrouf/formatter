use std::error::Error;
use std::fs;
use std::path::Path;

// Function to load prompt templates
pub fn load_prompt(file_path: &str) -> Result<String, Box<dyn Error>> {
    if !Path::new(file_path).exists() {
        return Err(format!("Prompt file not found: {}", file_path).into());
    }
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}
