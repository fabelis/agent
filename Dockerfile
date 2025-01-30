# Use Rust latest stable as base image
FROM rust:1.83-slim AS builder

# Install OpenSSL development packages in builder stage
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY src src/

# Build the application
RUN cargo build --release && \
    ls -la target/release/

# Create a new stage with a minimal image
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/agent /usr/local/bin/app

# Copy .env file
COPY .env .
COPY ./characters ./characters
COPY ./config.json .

# Set environment variables from .env using the recommended format
ENV $(grep -v '^#' .env | xargs -d '\n' -I {} echo "{}") 

# Run the application with custom command
CMD ["/usr/local/bin/app", "--character", "shinji02.json"]