FROM rust:1.75 as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    libfontconfig1-dev \
    pkg-_config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libfontconfig1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/qqn-optimizer /usr/local/bin/

# Create a non-root user
RUN useradd -r -s /bin/false qqnuser
USER qqnuser

ENTRYPOINT ["qqn-optimizer"]