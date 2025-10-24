# Use the official Rust image with a specific version
FROM rust:1.82-slim

# Set working directory
WORKDIR /app

# Copy Cargo.toml first for better caching
COPY Cargo.toml ./

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Expose port
EXPOSE 4000

# Run the application
CMD ["./target/release/funify-backend"]