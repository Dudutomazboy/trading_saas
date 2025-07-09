# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/trading-saas-backend /app/

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Create a non-root user
RUN useradd -r -s /bin/false appuser
USER appuser

EXPOSE 8000

CMD ["./trading-saas-backend"]
