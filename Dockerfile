# Define build arguments
ARG RUST_VERSION=1.85.0

# Stage 1: Builder
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/src/files-api

# Install dependencies for building
RUN apt-get update && apt-get install -y --no-install-recommends \
    binutils libpq-dev curl pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code
COPY . .

# Fetch dependencies
RUN cargo fetch --locked

# Build the application
RUN cargo build --release --locked && \
    strip target/release/files-api

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates wget libpq5 curl libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and group
RUN groupadd --system files-api && useradd --no-log-init --system -g files-api files-api

# Copy the compiled binary
COPY --from=builder --chown=files-api:files-api /usr/src/files-api/target/release/files-api /usr/local/bin/files-api

# Set permissions
USER files-api

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/files-api"]
CMD []
