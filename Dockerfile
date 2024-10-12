
#################
## build stage ##
#################
FROM rust:1-slim-bookworm AS builder

# Set environment variables to avoid interactive prompts
# ENV DEBIAN_FRONTEND=noninteractive

########################
## TEMPORARY DISABLED ##
## TO TEST RUST IMG   ##
########################
# RUN apt-get update && \
#   apt-get install -y \
#   curl \
#   build-essential \
#   libssl-dev \
#   pkg-config \
#   git \
#   redis-server \
#   awscli \
#   ffmpeg \
#   autoconf \
#   automake \
#   build-essential \
#   cmake \
#   git-core \
#   libass-dev \
#   libfreetype6-dev \
#   libgnutls28-dev \
#   libmp3lame-dev \
#   libsdl2-dev \
#   libtool \
#   libva-dev \
#   libvdpau-dev \
#   libvorbis-dev \
#   libxcb1-dev \
#   libxcb-shm0-dev \
#   libxcb-xfixes0-dev \
#   meson \
#   ninja-build \
#   pkg-config \
#   texinfo \
#   wget \
#   yasm \
#   zlib1g-dev \
#   ca-certificates && \
#   apt-get clean && rm -rf /var/lib/apt/lists/*

# Set PKG_CONFIG_PATH
# RUN export PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" \
#   --prefix="$HOME/ffmpeg_build" \
#   --extra-cflags="-I$HOME/ffmpeg_build/include" \
#   --extra-ldflags="-L$HOME/ffmpeg_build/lib"

# Install Rust
# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /code

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

# Copy the project files
COPY src src

# Compile the project
RUN cargo build --release

# Run stage
FROM bitnami/minideb:bookworm
WORKDIR /app

# cocpy the built binary from the build stage
COPY --from=builder /code/target/release/be-meeturbait /docker_app

# set user to non-root
USER 1001

# indicate what port the container will listen on at runtime.
EXPOSE 8080

# run the application
CMD ["/docker_app"]