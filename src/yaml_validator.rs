use graflog::app_log;
use std::error::Error;

/// Validates and fixes common YAML indentation issues
pub fn validate_and_fix_yaml(yaml_content: &str) -> Result<String, Box<dyn Error>> {
    // Parse the YAML content to check for errors
    match serde_yaml::from_str::<serde_yaml::Value>(yaml_content) {
        Ok(_) => {
            app_log!(info, "YAML validation successful");
            Ok(yaml_content.to_string())
        }
        Err(e) => {
            app_log!(warn, "YAML validation failed: {}", e);
            app_log!(info, "Attempting to fix YAML indentation issues");

            // Fix indentation issues
            let fixed_yaml = fix_yaml_indentation(yaml_content);

            // Validate the fixed YAML
            match serde_yaml::from_str::<serde_yaml::Value>(&fixed_yaml) {
                Ok(_) => {
                    app_log!(info, "YAML fixed successfully");
                    Ok(fixed_yaml)
                }
                Err(e) => {
                    app_log!(warn, "Could not fix YAML automatically: {}", e);
                    Err(format!("Failed to fix YAML: {}", e).into())
                }
            }
        }
    }
}

/// Fixes common YAML indentation issues
fn fix_yaml_indentation(yaml_content: &str) -> String {
    let lines: Vec<&str> = yaml_content.lines().collect();
    let mut fixed_lines = Vec::new();
    let mut current_indent_level = 0;
    let indent_size = 2; // Standard YAML indent size

    // Detect root level items (usually api_groups:)
    let root_items: Vec<&str> = lines
        .iter()
        .filter(|line| {
            !line.trim().is_empty()
                && !line.trim().starts_with('-')
                && !line.trim().starts_with('#')
        })
        .filter(|line| !line.trim_start().starts_with('-'))
        .map(|line| line.trim().split(':').next().unwrap_or(""))
        .filter(|item| !item.is_empty())
        .collect();

    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            fixed_lines.push(String::new());
            continue;
        }

        // Handle root level items
        if root_items.contains(&trimmed.split(':').next().unwrap_or(""))
            && !trimmed.starts_with('-')
        {
            current_indent_level = 0;
            fixed_lines.push(trimmed.to_string());
            continue;
        }

        // Handle list items
        if trimmed.starts_with('-') {
            // Determine if this is a new list start or continuation
            if !fixed_lines.is_empty() {
                let prev_line = fixed_lines.last().unwrap().trim();
                if prev_line.ends_with(':') {
                    // This is a list item after a property, indent more
                    current_indent_level += 1;
                }
            }

            let spaces = " ".repeat(current_indent_level * indent_size);
            fixed_lines.push(format!("{}{}", spaces, trimmed));
            continue;
        }

        // Handle properties inside list items
        if trimmed.contains(':') && !trimmed.ends_with(':') {
            let spaces = " ".repeat((current_indent_level + 1) * indent_size);
            fixed_lines.push(format!("{}{}", spaces, trimmed));
        } else if trimmed.ends_with(':') {
            // Property that will contain more nested items
            let spaces = " ".repeat(current_indent_level * indent_size);
            fixed_lines.push(format!("{}{}", spaces, trimmed));
        } else {
            // Other lines, preserve as is but with proper indentation
            let spaces = " ".repeat(current_indent_level * indent_size);
            fixed_lines.push(format!("{}{}", spaces, trimmed));
        }
    }

    fixed_lines.join("\n")
}

// Alternative approach: Use YAML parser to properly format the YAML
// pub fn reformat_yaml(yaml_content: &str) -> Result<String, Box<dyn Error>> {
//     // First try to parse the YAML
//     let yaml_value: serde_yaml::Value = match serde_yaml::from_str(yaml_content) {
//         Ok(value) => value,
//         Err(e) => {
//             // If parsing fails, try basic fixing first
//             let fixed_yaml = fix_yaml_indentation(yaml_content);
//             match serde_yaml::from_str(&fixed_yaml) {
//                 Ok(value) => value,
//                 Err(_) => return Err(format!("Failed to parse YAML: {}", e).into()),
//             }
//         }
//     };
//
//     // Then convert back to a properly formatted string
//     let formatted_yaml = serde_yaml::to_string(&yaml_value)?;
//     Ok(formatted_yaml)
// }
