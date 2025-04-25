# Containerfile for Podman – Rust + ESP32/ESP-IDF + QEMU environment
# Base image served from Microsoft’s ACR

FROM mcr.microsoft.com/devcontainers/base:jammy

# ─── 1) System Dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
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
        openocd \
        libglib2.0-dev \
        libpixman-1-dev \
        libslirp-dev \
        libsdl2-2.0-0 \
    && rm -rf /var/lib/apt/lists/*

# ─── 2) Rust Toolchain
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- \
        -y --no-modify-path --default-toolchain stable && \
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

# ─── 3) Flash & Debug Tools
RUN cargo install cargo-espflash ldproxy

# Install Probe-RS for `cargo embed`
RUN curl --proto '=https' --tlsv1.2 -LsSf \
        https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh \
    | sh

# ─── 4) espup (Rust bindings & Xtensa toolchain)
RUN cargo install espup

# ─── 5) Clone & Install ESP-IDF v5.4.1
ARG ESP_IDF_VERSION=5.4.1
ENV ESP_IDF_PATH=/opt/esp-idf

# Clone ESP-IDF
RUN git clone --branch v${ESP_IDF_VERSION} --recursive \
        https://github.com/espressif/esp-idf.git ${ESP_IDF_PATH}

# Set working directory and run install script
WORKDIR ${ESP_IDF_PATH}
RUN ./install.sh # Or specific chips: ./install.sh esp32,esp32s3,esp32c3

# Optional: Reset WORKDIR if subsequent commands expect a different default
WORKDIR /workspace

# Instruct esp-idf-sys to use the system-installed IDF
# ENV ESP_IDF_TOOLS_INSTALL_DIR=SYSTEM
ENV PATH=${ESP_IDF_PATH}/tools:$PATH

# ─── 6) Install Xtensa-Rust Toolchain
# Pin the Rust (Xtensa) toolchain version if needed via XTENSA_TOOLCHAIN_VERSION
ARG XTENSA_TOOLCHAIN_VERSION=stable
RUN espup install

# ─── 7) QEMU Binaries for ESP32 & RISC-V
ARG QEMU_RELEASE_TAG=esp-develop-9.2.2-20250228
ARG QEMU_VERSION_STR=esp_develop_9.2.2_20250228
ARG QEMU_HOST_ARCH=x86_64-linux-gnu
ARG QEMU_INSTALL_DIR=/opt/qemu-esp

RUN mkdir -p /tmp/qemu-download ${QEMU_INSTALL_DIR} && \
    cd /tmp/qemu-download && \
    for CORE in xtensa riscv32; do \
        TARBALL="qemu-${CORE}-softmmu-${QEMU_VERSION_STR}-${QEMU_HOST_ARCH}.tar.xz"; \
        wget "https://github.com/espressif/qemu/releases/download/${QEMU_RELEASE_TAG}/${TARBALL}"; \
        tar -xf ${TARBALL} -C ${QEMU_INSTALL_DIR} --strip-components=1; \
    done && \
    rm -rf /tmp/qemu-download

ENV PATH=${QEMU_INSTALL_DIR}/bin:/usr/local/cargo/bin:${ESP_IDF_PATH}/tools:$PATH

# ─── 8) Workspace
WORKDIR /workspace
# At runtime: podman run --rm -it -v "$(pwd)":/workspace rust-esp-starter

CMD ["/bin/bash"]
