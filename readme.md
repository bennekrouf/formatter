# YAML Formatter HTTP Service

A simple HTTP service that takes a YAML file as input and returns a formatted YAML file as output, following a specific template structure.

## Features

- HTTP endpoint for file uploads
- Formats YAML using Ollama API
- Returns formatted YAML file
- Basic error handling and logging

## Requirements

- Rust 1.75+
- Ollama running locally on port 11434

## Setup

1. Ensure you have the required template files:

   - `template.yaml` - The template structure for formatting
   - `prompt/system_prompt.txt` - System prompt for Ollama
   - `prompt/user_prompt.txt` - User prompt for Ollama

2. Build the service:

   ```bash
   cargo build --release
   ```

3. Run the service:

   ```bash
   ./target/release/ai-uploader
   ```

The service will start on port 8080.

## Usage

- Upload a file for formatting: `POST /format-yaml`
- Check service status: `GET /health`

See `test.sh` for an example of how to use the service with curl.
