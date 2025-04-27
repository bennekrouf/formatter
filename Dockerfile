# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy source file to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build only dependencies to cache them
RUN cargo build --release

# Remove dummy source file and any artifacts
RUN rm -rf src target/release/deps/ai_uploader*

# Copy actual source code
COPY src/ ./src/
COPY prompt/ ./prompt/
COPY template.yaml ./

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder
COPY --from=builder /app/target/release/ai-uploader /app/ai-uploader

# Copy required files
COPY --from=builder /app/prompt/ /app/prompt/
COPY --from=builder /app/template.yaml /app/

# Set environment variables
ENV RUST_LOG=info

# Expose the port
EXPOSE 8080

# Start the application
CMD ["/app/ai-uploader"]
