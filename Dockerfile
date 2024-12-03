# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock resources .env ./

# Copy the source code
COPY src src

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM ubuntu:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/promptier .
COPY --from=builder /app/.env .

# Set the startup command to run the binary
CMD ["./promptier"]
