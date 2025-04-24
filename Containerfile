# Containerfile for Podman – generic Rust + ESP32/ESP-IDF environment
# Uses Microsoft Container Registry (ACR) and supports desktop, mobile, and ESP targets

FROM mcr.microsoft.com/devcontainers/base:jammy

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates git curl build-essential cmake python3 python3-pip python3-venv \
    libssl-dev pkg-config wget unzip openocd && \
    rm -rf /var/lib/apt/lists/*

# Install Rust via rustup and add targets
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain stable && \
    rustup target add \
      x86_64-unknown-linux-gnu \
      x86_64-pc-windows-gnu \
      x86_64-pc-windows-msvc \
      x86_64-apple-darwin \
      aarch64-unknown-linux-gnu \
      aarch64-apple-darwin \
      aarch64-linux-android \
      armv7-linux-androideabi \
      aarch64-apple-ios \
      wasm32-unknown-unknown

# Install espup for Xtensa (ESP32) support
RUN cargo install espup && \
    espup install

# Clone and install ESP-IDF
RUN git clone --recursive https://github.com/espressif/esp-idf.git /opt/esp-idf && \
    cd /opt/esp-idf && ./install.sh

ENV ESP_IDF_PATH=/opt/esp-idf
ENV PATH=$ESP_IDF_PATH/tools:$PATH

# Workspace directory: mount your project here
WORKDIR /workspace
# COPY . /workspace

# Default to interactive shell: run 'cargo build', 'cargo espflash', etc.
CMD ["/bin/bash"]
