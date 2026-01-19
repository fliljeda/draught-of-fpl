# Multi-stage Dockerfile for Draught of FPL
# Stage 1: Builder Dependencies
FROM rust:1.92 AS deps

WORKDIR /build

# Install build dependencies (cached separately from builds)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Stage 2: Builder
FROM deps AS builder

# Copy manifest files
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src
COPY www ./www

# Build in release mode with cache mounts
# This caches the target directory and cargo registry between builds
RUN --mount=type=cache,target=/build/target \
    cargo build --release && \
    cp /build/target/release/draught-of-fpl /draught-of-fpl

# Stage 3: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Create non-root user
RUN useradd -m -u 1000 appuser

# Copy binary from builder
COPY --from=builder --chown=appuser:appuser /draught-of-fpl /app/draught-of-fpl
COPY --from=builder --chown=appuser:appuser /build/www /app/www

# Switch to non-root user
USER appuser

# Set environment variables with defaults
# These can be overridden at runtime
ENV DOF_LEAGUE_ID=2481
ENV DOF_LOCAL_FETCH=false
ENV DOF_LOCAL_URL=
ENV DOF_ASSET_PATH=/app/www/vue
ENV DOF_SERVER_PORT=80

# Run the application
ENTRYPOINT ["/app/draught-of-fpl"]
