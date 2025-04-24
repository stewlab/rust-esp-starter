//! # led_blinking_app
//! Blinks an LED on the ESP32 using esp-idf-hal v0.45.2.
//!  

// Link the ESP-IDF C runtime and apply necessary patches (via binstart feature).
use esp_idf_sys as _;

// --- Required Imports ---
use esp_idf_hal::delay::FreeRtos; // FreeRTOS-based delay
use esp_idf_hal::gpio::PinDriver; // GPIO PinDriver for configuration
use esp_idf_hal::peripherals::Peripherals; // Access to chip peripherals

use anyhow::Result;
use esp_idf_svc::log::EspLogger; // ESP-IDF logger implementation
use log::info; // Logging facade // <-- Import anyhow::Result for error handling

/// Runs the LED blinking loop:  
/// toggles the LED on/off every second and logs the state.
pub fn run() -> Result<()> {
    // <-- Use anyhow::Result here
    // Initialize the ESP-IDF logger. Outputs to serial console.
    EspLogger::initialize_default();
    info!("Logger initialized");

    // Take ownership of ESP32 peripherals.
    let peripherals = Peripherals::take()?;
    info!("Peripherals taken");

    // --- GPIO Configuration (using PinDriver) ---
    // IMPORTANT: Change `peripherals.pins.gpio4` if your LED is on a different pin!
    info!("Configuring GPIO for LED...");
    // *** USE THIS LINE FOR v0.45.2 ***
    let mut led_pin = PinDriver::output(peripherals.pins.gpio4)?;
    info!("GPIO configuration complete. Starting blink loop...");

    // --- Main Application Loop ---
    loop {
        // Turn LED ON
        led_pin.set_high()?;
        info!("LED ON");
        FreeRtos::delay_ms(1000); // Wait 1 second

        // Turn LED OFF
        led_pin.set_low()?;
        info!("LED OFF");
        FreeRtos::delay_ms(1000); // Wait 1 second
    }
    // Unreachable
}
