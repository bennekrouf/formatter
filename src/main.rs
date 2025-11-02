use actix_multipart::{Field, Multipart};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use anyhow::Result;
use format_yaml_with_ollama::format_yaml_with_cohere;
use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;
use graflog::{app_log, init_logging};
use std::env;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;
use uuid::Uuid;

mod extract_yaml;
mod format_yaml_with_ollama;
mod load_prompt;
mod models;
mod yaml_validator;

struct AppState {
    template_path: String,
    system_prompt_path: String,
    user_prompt_path: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables at startup
    dotenv::dotenv().ok();
    init_logging!("/var/log/api0.log", "api0", "ai-uploader");

    // Parse command line arguments - optional "server" subcommand
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] != "server" {
        app_log!(info, "Usage: {} [server]", args[0]);
        std::process::exit(1);
    }

    app_log!(info, "Starting YAML formatter HTTP service");

    let port = env::var("ROCKET_PORT")
        .or_else(|_| env::var("PORT"))
        .unwrap_or_else(|_| "6666".to_string())
        .parse::<u16>()
        .map_err(|e| {
            app_log!(error, "Invalid port number: {}", e);
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid port number")
        })?;

    // Get base path from environment or use current directory
    let base_path = if let Ok(config_path) = env::var("CONFIG_PATH") {
        // Production path provided via environment variable
        if config_path.ends_with(".yaml") {
            std::path::Path::new(&config_path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        } else {
            config_path
        }
    } else {
        // Development: use current directory
        ".".to_string()
    };

    // Define file paths relative to base path
    let template_file_path = format!("{}/template.yaml", base_path);
    let system_prompt_path = format!("{}/prompt/system_prompt.txt", base_path);
    let user_prompt_path = format!("{}/prompt/user_prompt.txt", base_path);

    app_log!(info, "Using base path: {}", base_path);
    app_log!(info, "Template file: {}", template_file_path);
    app_log!(info, "System prompt: {}", system_prompt_path);
    app_log!(info, "User prompt: {}", user_prompt_path);
    app_log!(info, "Starting server on port: {}", port);

    // Ensure the prompt directory exists
    let prompt_dir = format!("{}/prompt", base_path);
    if !Path::new(&prompt_dir).exists() {
        std::fs::create_dir_all(&prompt_dir).expect("Failed to create prompt directory");
        app_log!(info, "Created prompt directory");
    }

    // Check if prompt files exist
    for (path, name) in [
        (&system_prompt_path, "system prompt"),
        (&user_prompt_path, "user prompt"),
        (&template_file_path, "template file"),
    ] {
        if !Path::new(path).exists() {
            app_log!(error, "{} file not found at {}", name, path);
            panic!("Missing {} file", name);
        }
    }

    let app_state = web::Data::new(AppState {
        template_path: template_file_path,
        system_prompt_path,
        user_prompt_path,
    });

    // Start HTTP server with dynamic port
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/format-yaml", web::post().to(format_yaml_handler))
            .route("/health", web::get().to(health_check))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Service is running")
}

async fn save_field(field: Field) -> Result<String, Error> {
    let content_disposition = field.content_disposition();
    let filename = content_disposition
        .and_then(|cd| cd.get_filename())
        .unwrap_or("upload.txt");
    let filepath = format!(
        "/tmp/{}-{}",
        Uuid::new_v4(),
        sanitize_filename::sanitize(filename)
    );

    app_log!(debug, "Saving uploaded file to {}", filepath);

    let mut temp_file = std::fs::File::create(&filepath)?;
    let mut bytes = web::BytesMut::new();

    let mut field_stream = field;
    while let Some(chunk) = field_stream.next().await {
        let data = chunk?;
        bytes.extend_from_slice(&data);
        temp_file.write_all(&data)?;
    }

    temp_file.flush()?;
    Ok(filepath)
}

async fn format_yaml_handler(
    multipart: Multipart,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let mut input_path = None;

    // Process the multipart form data
    let mut multipart_data = multipart;

    app_log!(info, "Processing uploaded file");
    'field_loop: while let Ok(Some(field)) = multipart_data.try_next().await {
        if field.name() == Some("file") {
            input_path = Some(save_field(field).await?);
            break 'field_loop;
        }
    }

    let input_file_path = input_path.ok_or_else(|| {
        app_log!(error, "No file was uploaded");
        actix_web::error::ErrorBadRequest("No file was uploaded")
    })?;

    app_log!(info, "Processing file: {}", input_file_path);

    // Process the uploaded file
    match format_yaml_with_cohere(
        // renamed function
        &input_file_path,
        &app_state.template_path,
        &app_state.system_prompt_path,
        &app_state.user_prompt_path,
    )
    .await
    {
        Ok(formatted_yaml) => {
            // Prepare the response
            app_log!(info, "Successfully formatted YAML");

            // Create a temporary file for the response
            let mut temp_file = NamedTempFile::new()?;
            temp_file.write_all(formatted_yaml.as_bytes())?;
            temp_file.flush()?;

            // Clean up the input file
            if let Err(e) = std::fs::remove_file(&input_file_path) {
                app_log!(error, "Failed to remove temporary input file: {}", e);
            }

            // Return the formatted YAML
            Ok(HttpResponse::Ok()
                .content_type("application/yaml")
                .append_header((
                    "Content-Disposition",
                    "attachment; filename=\"formatted_output.yaml\"",
                ))
                .body(formatted_yaml))
        }
        Err(e) => {
            app_log!(error, "Error formatting YAML: {}", e);
            // Clean up the input file
            if let Err(cleanup_err) = std::fs::remove_file(&input_file_path) {
                app_log!(
                    error,
                    "Failed to remove temporary input file: {}",
                    cleanup_err
                );
            }

            Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e)))
        }
    }
}
