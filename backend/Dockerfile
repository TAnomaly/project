# Use the official Rust image with a specific version
FROM rust:1.75-slim

# Set working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached if Cargo.toml doesn't change)
RUN cargo build --release

# Remove the dummy main.rs
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Expose port
EXPOSE 4000

# Run the application
CMD ["./target/release/funify-backend"]