# Use the official Rust image as a base
FROM rust:latest

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    alsa-utils \
    zenity \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the Rust project files
COPY . .

# Ensure the sounds directory is included
RUN mkdir -p /app/sounds
COPY sounds /app/sounds

# Build the Rust application
RUN cargo build --release

# Set the entry point to the compiled binary
CMD ["./target/release/run-alarm"]
