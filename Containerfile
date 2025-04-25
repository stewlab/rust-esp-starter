# Containerfile for Podman – generic Rust + ESP32/ESP-IDF environment
# Uses Microsoft Container Registry (ACR) and supports desktop, mobile, and ESP targets

FROM mcr.microsoft.com/devcontainers/base:jammy

# Install system dependencies (including libudev-dev for libudev-sys)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    git \
    curl \
    build-essential \
    cmake \
    python3 \
    python3-pip \
    python3-venv \
    flex \
    bison \
    ninja-build \
    ccache \
    libffi-dev \
    dfu-util \
    libusb-1.0-0 \
    libudev-dev \
    libssl-dev \
    pkg-config \
    wget \
    unzip \
    openocd && \
    rm -rf /var/lib/apt/lists/*

# Install Rust via rustup and add all major desktop/mobile/WebAssembly targets
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

# Install helper Cargo tools: cargo-espflash
RUN cargo install cargo-espflash

# Install Probe-RS tools for `cargo embed`
RUN curl --proto '=https' --tlsv1.2 -LsSf \
      https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh \
    | sh

ARG ESP_IDF_VERSION=5.4.1
ENV ESP_IDF_VERSION=${ESP_IDF_VERSION}

ENV ESP_IDF_PATH=/opt/esp-idf

# Install ESP-IDF
RUN git clone --recursive https://github.com/espressif/esp-idf.git ${ESP_IDF_PATH} && \
# RUN git clone --branch v${ESP_IDF_VERSION} --recursive \
#       https://github.com/espressif/esp-idf.git ${ESP_IDF_PATH} && \
    cd ${ESP_IDF_PATH} && \
    # Explicitly check out the version TAG
    echo "Checking out IDF tag v${ESP_IDF_VERSION}..." && \
    git checkout "v${ESP_IDF_VERSION}" && \
    # Ensure submodules are updated for the checked-out tag
    git submodule update --init --recursive && \
    # Run the install script (optionally specify targets: ./install.sh esp32,esp32s3)
    ./install.sh

ENV PATH=${ESP_IDF_PATH}/tools:$PATH

# Install espup
RUN cargo install espup

# Use espup to install Xtensa toolchain & Rust bindings for ESP-IDF
RUN espup install --std

# Install ldproxy
RUN cargo install ldproxy

# ESP_IDF_PATH is set automatically by espup (in ~/.espressif/)
# ENV ESP_IDF_PATH=/root/.espressif/esp-idf@5.4.1

# Workspace directory: mount your project here
WORKDIR /workspace
# Note: do NOT COPY your code—mount via `-v "$(pwd)":/workspace` when running.

# Drop into a shell for interactive development
CMD ["/bin/bash"]
