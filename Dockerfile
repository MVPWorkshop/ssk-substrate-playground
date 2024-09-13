# Use an official Rust image based on Debian Bullseye (or the same base image)
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the project files
COPY . .

# Build the project in release mode
RUN cargo build --release

# Use the same base image as the builder to avoid version mismatch
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/substrate-runtime-builder /usr/local/bin/substrate-runtime-builder

# Expose the necessary ports
EXPOSE 8080

# Run the binary
CMD ["substrate-runtime-builder"]
