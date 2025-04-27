// Helper function to extract only the YAML part from the response
pub fn extract_yaml(response: &str) -> String {
    // If response has ```yaml ... ``` format, extract only the YAML part
    if response.contains("```yaml") && response.contains("```") {
        let start = response.find("```yaml").unwrap_or(0) + 7;
        let end = response[start..]
            .find("```")
            .unwrap_or(response.len() - start)
            + start;
        response[start..end].trim().to_string()
    } else if response.contains("```") {
        // If it has just code blocks without yaml specification
        let start = response.find("```").unwrap_or(0) + 3;
        let end = response[start..]
            .find("```")
            .unwrap_or(response.len() - start)
            + start;
        response[start..end].trim().to_string()
    } else {
        // Assume the entire response is YAML
        response.trim().to_string()
    }
}
