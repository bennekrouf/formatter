# YAML Formatter HTTP Service

A simple HTTP service that takes a YAML file as input and returns a formatted YAML file as output, following a specific template structure using Cohere's AI API.

## Features

- HTTP endpoint for file uploads
- Formats YAML using Cohere API
- Returns formatted YAML file
- Basic error handling and logging
- Environment variable configuration for API keys

## Requirements

- Rust 1.75+
- Cohere API key

## Setup

### 1. Environment Configuration

Create a `.env` file in the project root:

```env
COHERE_API_KEY=your_cohere_api_key_here
```

### 2. Required Template Files

Ensure you have the required template files:

- `template.yaml` - The template structure for formatting
- `prompt/system_prompt.txt` - System prompt for Cohere
- `prompt/user_prompt.txt` - User prompt for Cohere

### 3. Build and Run

Build the service:

```bash
cargo build --release
```

Run the service:

```bash
./target/release/ai-uploader
```

The service will start on port 8080.

## API Endpoints

- `POST /format-yaml` - Upload a file for formatting
- `GET /health` - Check service status

## Usage

### Basic curl example

```bash
curl -X POST \
  -F "file=@divess.yaml" \
  http://localhost:8080/format-yaml \
  -o formatted_output.yaml
```

### Health check

```bash
curl http://localhost:8080/health
# Returns: "Service is running"
```

## Input File Format

The service expects YAML files containing API endpoint definitions. Example input:

```yaml
Service 1:
get_themes_list
description: "Return all categories and information"

Service 2:
upload_document
description: "Upload a document with event ID"

name: "file"
description: "File to upload"
required: true
```

## Output Format

The service returns properly formatted YAML following the template structure:

```yaml
api_groups:
  - name: "Service 1"
    description: "Service API endpoints"
    base: "https://api.example.com"
    endpoints:
      - text: "get_themes_list"
        description: "Return all categories and information"
        verb: "GET"
        path: "/themes"
        parameters: []
```

## Environment Variables

- `COHERE_API_KEY` - Required: Your Cohere API key
- `RUST_LOG` - Optional: Set logging level (debug, info, warn, error)
