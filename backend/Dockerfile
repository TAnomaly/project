# Use the official Rust image as the base image
FROM rust:1.75-slim as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached if Cargo.toml doesn't change)
RUN cargo build --release

# Remove the dummy main.rs
RUN rm src/main.rs

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/funify-backend /app/funify-backend

# Change ownership of the binary
RUN chown appuser:appuser /app/funify-backend

# Switch to the non-root user
USER appuser

# Expose the port
EXPOSE 4000

# Set environment variables
ENV RUST_LOG=info
ENV PORT=4000

# Run the application
CMD ["./funify-backend"]
