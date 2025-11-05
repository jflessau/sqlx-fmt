FROM rust:1.70-slim as builder

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy source code
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build the release binary
RUN cargo build --release

# Runtime stage with Python for sqruff
FROM python:3.11-slim

# Install sqruff
RUN pip install sqruff

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/sqlx-fmt /usr/local/bin/sqlx-fmt

# Make sure the binary is executable
RUN chmod +x /usr/local/bin/sqlx-fmt

# Set working directory
WORKDIR /workspace

# Default entrypoint
ENTRYPOINT ["/usr/local/bin/sqlx-fmt"]
