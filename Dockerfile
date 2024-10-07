# Use an official Ubuntu as the base image
FROM ubuntu:22.04

# Set environment variables to avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && \
  apt-get install -y \
  curl \
  build-essential \
  libssl-dev \
  pkg-config \
  git \
  redis-server \
  awscli \
  ffmpeg \
  ca-certificates && \
  apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /app

# Copy the project files
COPY . .

# Build the project
RUN cargo build --release

# Expose the port that Actix will run on
EXPOSE 8080

# Command to run the Actix app
CMD ["./target/release/your-actix-app"]
