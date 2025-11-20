# Multi-stage build for reproducible and secure builds
FROM rust:1.75-slim AS builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --target x86_64-unknown-linux-musl && rm -rf target/x86_64-unknown-linux-musl/release/deps/herding*

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage - minimal image
FROM alpine:3.18

# Install runtime dependencies only
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -g 1001 -S appuser && \
    adduser -u 1001 -S appuser -G appuser

# Copy the binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/herding-cats-rust /usr/local/bin/

# Set ownership
RUN chown appuser:appuser /usr/local/bin/herding-cats-rust

# Switch to non-root user
USER appuser

# Expose port if needed (adjust as necessary)
# EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /usr/local/bin/herding-cats-rust --version || exit 1

# Set the binary as entrypoint
ENTRYPOINT ["/usr/local/bin/herding-cats-rust"]