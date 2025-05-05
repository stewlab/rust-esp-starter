# rust-esp-starter

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Rust starter for ESP32 integrating Espressif’s ESP-IDF with the esp-rs ecosystem.**  
This project was originally generated from [esp-rs/esp-idf-template](https://github.com/esp-rs/esp-idf-template).

---

## Features

- Abstractions for ESP32 peripherals
- Optional modules: display, graphics
- Examples: `hello_app`, `led_blinking_app`, `graphics_app`, etc.
- Integration with Cargo, `esp-idf-sys`, `esp-idf-hal`

---

## Prerequisites

- Podman (or Docker)
- Rust toolchain (stable)
- ESP32 toolchain

### Container Setup (Podman)

```bash
podman build -t rust-esp-env .
podman run --rm -it \
  -v "$(pwd)":/workspace:Z \
  -w /workspace \
  rust-esp-env
```

Inside the container, `/workspace` is your project root. Run commands like `cargo build` and `cargo espflash`, then exit.

---

## Getting Started

1. **Clone the repository**

   ```bash
   git clone https://github.com/stewlab/rust-esp-starter.git
   cd rust-esp-starter
   ```

2. **(Optional) Build and launch the development container**

   ```bash
   # Build the container image (run once, unless Containerfile changes)
   podman build -t rust-esp-env .

   # Start a dev container
   podman run --rm -it \
     -v "$(pwd)":/workspace:Z \
     -w /workspace \
     rust-esp-env
   ```

   Inside the container, `/workspace` is your project root.

3. **Build the project**

   ```bash
   cargo build
   ```

4. **Configure** *(optional)*

   * Customize `sdkconfig.defaults` for your board

5. **Flash & Monitor:**

   ```bash
   cargo espflash flash -p /dev/ttyUSB0
   espflash monitor
   ```

   Press **Ctrl+R** to reset the device.

---

## Usage

* By default, `hello_app` runs. To use another example, edit `src/main.rs`:

  ```rust
  fn main() {
      crate::apps::led_blinking_app::run().unwrap();
  }
  ```

* Enable features with Cargo:

  ```bash
  cargo espflash --release --features "display-support graphics-support"
  ```

---

## Feature Flags

| Flag               | Description                 |
| ------------------ | --------------------------- |
| `display-support`  | Display & backlight modules |
| `graphics-support` | Embedded‑graphics examples  |

---

## Contributing

Contributions welcome:

1. Fork the repo
2. Create a branch
3. Run `cargo fmt`, add tests/docs
4. Open a PR to `main`

---

## Tested Devices

* **ESP32-2432S028** ([AliExpress](https://www.aliexpress.com/item/1005006470918908.html))

Tested on Ubuntu 24.04.2 via Podman

---

## License

MIT License. See [LICENSE](LICENSE).
