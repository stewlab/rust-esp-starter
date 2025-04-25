# rust-esp-starter

![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)

**Battery-optimized Rust starter for ESP32**, seamlessly integrating Espressif’s ESP-IDF and the esp-rs ecosystem to accelerate development from prototype to production.

This project was originally generated from [esp-rs/esp-idf-template](https://github.com/esp-rs/esp-idf-template).

---


## 📋 Table of Contents

- [Features](#-features)
- [Prerequisites](#-prerequisites)
- [Getting Started](#-getting-started)
- [Usage](#-usage)
- [Feature Flags](#-feature-flags)
- [Project Structure](#-project-structure)
- [Documentation](#-documentation)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🔧 Features

- Zero-cost abstractions for ESP32 peripherals
- Optional modules for display, graphics, and RTOS shell
- Multiple example apps: `hello_app`, `led_blinking_app`, `graphics_app`, and more
- Works seamlessly with Cargo, `esp-idf-sys`, and `esp-idf-hal`
- CI integration via GitHub Actions
- Well-documented codebase with Rust doc comments

---

## 🛠 Prerequisites

> 🎯 **Container Setup (Podman)**  
> The project includes a `Containerfile` that installs Rust, ESP-IDF, and all necessary tools. It’s designed to be **generic**: mount **any** Rust project into `/workspace` and build/debug it inside the container.
>
> Build and run:
>
> ```bash
> # build image from Containerfile
> podman build -t rust-esp-starter .
> 
> # start a development container (replace $(pwd) w/ your project root)
> podman run --rm -it -v "$(pwd)":/workspace rust-esp-starter
> # or, if getting permission errors 
> podman run --rm -it -v "$(pwd)":/workspace:Z -w /workspace rust-esp-starter
> ```
>
> Inside, your working directory is `/workspace`. Run your usual commands (e.g., `cargo build`, `cargo espflash`, etc.).
>
> Exit with `exit` when done.

<!-- This project includes a `Containerfile` that installs all required tools:

- Rust toolchain and targets (desktop, mobile, embedded)
- ESP-IDF (via official install.sh)
- Xtensa toolchain for ESP32 (`espup`)
- Flashing and debug tools: `cargo-espflash`, `cargo-embed`
- Optional OpenOCD (JTAG support) -->
---

## 🚀 Getting Started



1. **Clone the template**:

   ```bash
   git clone https://github.com/stewlab/rust-esp-starter.git
   cd rust-esp-starter
   ```

2. **🚀 Build & Launch the Container**  

   > See [Prerequisites](#-prerequisites)

   > **Note**: remaining steps should be performed inside the development container


3. **Configure your board** (optional):  
   Copy and edit `sdkconfig.defaults` for your ESP32 module.

4. **Set up your environment**:  
   Source environment scripts to configure ESP-IDF and Rust targets:
   ```bash
   . $HOME/export-esp.sh       # Exports custom toolchain vars
   . $ESP_IDF_PATH/export.sh   # ESP-IDF environment variables
   ```

5. **Build the Project**:  
   ```bash
   cargo build
   ```

6. **Flash to the Device**:  
   ```bash
   cargo espflash flash -p /dev/ttyUSB0
   ```

7. **Monitor Serial Output**:  
   ```bash
   espflash monitor
   ```
   Use `Ctrl+R` to reset the device.
---

## ⚙️ Usage

By default, `hello_app` runs. To enable other apps, edit `src/main.rs`:

```rust
fn main() {
    // Run the LED blinking example
    crate::apps::led_blinking_app::run().unwrap();
}
```

You can also pass cargo features:

```bash
cargo espflash --release --features "display-support graphics-support"
```

---

## 🏷 Feature Flags

| Feature            | Description                           |
| ------------------ | ------------------------------------- |
| `display-support`  | Enables display and backlight modules |
| `graphics-support` | Enables embedded-graphics examples    |
| `rtos-shell`       | Enables RTOS shell application        |

---

## 🗂 Project Structure

```
├── Containerfile        # Podman container recipe
├── Cargo.toml
├── sdkconfig.defaults
├── src
│   ├── main.rs          # Entry point
│   └── apps             # Example applications
│       ├── hello_app.rs
│       ├── led_blinking_app.rs
│       └── ...
├── .github/workflows    # CI configuration
└── LICENSE              # MIT License
```

---

## 📖 Documentation

- Generate API docs with:

  ```bash
  cargo doc --open
  ```
- Browse `docs/` for additional guides (coming soon).

---

## 🤝 Contributing

Contributions are welcome! Please open issues or pull requests against `main`.  
Ensure all new code is documented, formatted (`cargo fmt`), and tested where applicable.

---

## ✅ Tested Devices

This starter has been tested with the following device(s):

| Device         | Link                                                                 |
|----------------|----------------------------------------------------------------------|
| ESP32-2432S028 | [AliExpress](https://www.aliexpress.com/item/1005006470918908.html) |

Tested on **Ubuntu 24.04.2 LTS** via **Podman container** with **Distrobox**.

## 📜 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

The license is also declared in `Cargo.toml`:

```toml
[package]
license = "MIT"
```
