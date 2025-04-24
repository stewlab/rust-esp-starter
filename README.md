# rust-esp-starter

![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)

A modern, battery-efficient Rust starter template for ESP32 microcontrollers using Espressif's ESP-IDF and the esp-rs ecosystem. Perfect for embedded Rust hacking, prototypes, and production builds.

---

## 📋 Table of Contents

- [Features](#-features)
- [Prerequisites](#-prerequisites)
- [Getting Started](#-getting-started)
- [Usage](#-usage)
- [Feature Flags](#-feature-flags)
- [Documentation](#-documentation)
- [Code Documentation & Style](#-code-documentation--style)
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

1. **Rust Toolchain**  
   - Install [rustup](https://rustup.rs/) (requires Rust ≥ 1.70).  
   - Set your default to `stable`:  
     ```bash
     rustup default stable
     ```
   - Add the Xtensa target for ESP32:  
     ```bash
     rustup target add xtensa-esp32-none-elf
     ```

2. **ESP-IDF**  
   - Clone Espressif's [ESP-IDF (v4.x or later)](https://github.com/espressif/esp-idf):  
     ```bash
     git clone --recursive https://github.com/espressif/esp-idf.git
     cd esp-idf
     ./install.sh
     ```  
   - Requires Python ≥ 3.7 and Git ≥ 2.20.

3. **ESP-IDF Environment**  
   - Source the environment variables in each shell session:  
     ```bash
     . $ESP_IDF_PATH/export.sh
     ```

4. **Cargo Flash & Debug Tools**  
   - Install helper tools:  
     ```bash
     cargo install cargo-espflash   # Flash & monitor
     cargo install cargo-embed      # Embedded debugger
     ```  
   - (Optional) Install OpenOCD for JTAG debugging:  
     ```bash
     sudo apt install openocd
     ```

5. **Xtensa GCC Toolchain**  
   - Ensure `xtensa-esp32-elf-gcc --version` shows v8.x+  
   - If not installed system-wide, configure via `RUST_TARGET_PATH`

6. **Development Environment (Optional)**  
   - Recommended: Visual Studio Code with [rust-analyzer](https://github.com/rust-lang/rust-analyzer) and Espressif extensions for ESP-IDF.

---

## 🚀 Getting Started

1. **Clone the template**:

   ```bash
   git clone https://github.com/yourusername/rust-esp-starter.git
   cd rust-esp-starter
   ```
2. **Configure your board** (optional):  
   Copy and edit `sdkconfig.defaults` for your ESP32 module.

3. **Set up your environment**:  
   Source environment scripts to configure ESP-IDF and Rust targets:
   ```bash
   . $HOME/export-esp.sh       # Exports custom toolchain vars
   . $ESP_IDF_PATH/export.sh   # ESP-IDF environment variables
   ```

4. **Build the Project**:  
   ```bash
   cargo build

   # or

   ESP_IDF_PATH="$HOME/esp/esp-idf" cargo build
   ```

5. **Flash to the Device**:  
   ```bash
   cargo espflash flash -p /dev/ttyUSB0
   ```

6. **Monitor Serial Output**:  
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

---

## 🏷 Feature Flags

| Feature            | Description                           |
| ------------------ | ------------------------------------- |
| `display-support`  | Enables display and backlight modules |
| `graphics-support` | Enables embedded-graphics examples    |
| `rtos-shell`       | Enables RTOS shell application        |

---

## 📖 Documentation

- Generate API docs with:

  ```bash
  cargo doc --open
  ```
- Browse `docs/` for additional guides (coming soon).

---

## 🧰 Code Documentation & Style

- **File-level docs:** Use `//!` comments at the top of each `*.rs` file.
- **Function docs:** Use `///` comments for public APIs (`pub fn run()`).
- **License:** Declared once in [LICENSE](LICENSE) and `Cargo.toml` (`license = "MIT"`). No per-file license header needed.
- **Formatting:** Run `cargo fmt` before commit.
- **Docs:** Generate with `cargo doc --open`.

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

