#![cfg(feature = "display-support")]

//! # display_backlight_app
//! Controls a display’s backlight on an ESP32.
//!  

use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _; // pull in ESP-IDF

// use esp_idf_hal::prelude::*;
use esp_idf_hal::peripherals::Peripherals;

// SPI
// use esp_idf_hal::spi::{SpiDriver, SpiDriverConfig};
// use esp_idf_hal::spi::config::MODE_0;
use esp_idf_hal::spi::{SpiDriver, SpiDriverConfig};

// ADC oneshot (we’ll just use the default ADC channel config
// which has a valid attenuation & width baked in)
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::{AdcChannelDriver, AdcDriver};

// GPIO
use esp_idf_hal::gpio::PinDriver;

use std::{thread, time::Duration};

/// Runs the display backlight demo:  
/// initializes the SPI, configures the display, and toggles the backlight.
pub fn run() -> anyhow::Result<()> {
    EspLogger::initialize_default();
    let mut peripherals = Peripherals::take().unwrap();

    // —————————————————
    // Light sensor (ADC1 on GPIO34)
    // —————————————————
    let adc = AdcDriver::new(peripherals.adc1)?;
    let mut light_pin = AdcChannelDriver::new(
        &adc,
        peripherals.pins.gpio34,
        &AdcChannelConfig::default(), // no need to pick DB_0 manually
    )?;
    let light_val = adc.read(&mut light_pin)?;
    println!("[touch] Light sensor: {}", light_val);

    // —————————————————
    // Backlight & RGB LEDs
    // —————————————————
    let mut backlight = PinDriver::output(peripherals.pins.gpio21)?;
    backlight.set_high()?; // turn the backlight on

    let mut red = PinDriver::output(peripherals.pins.gpio4)?;
    let mut green = PinDriver::output(peripherals.pins.gpio16)?;
    let mut blue = PinDriver::output(peripherals.pins.gpio17)?;
    red.set_low()?; // LEDs start off
    green.set_low()?;
    blue.set_low()?;

    // —————————————————
    // Touch controller on SPI3 (VSPI)
    // —————————————————
    let bus_cfg: SpiDriverConfig = Default::default();
    let touch_spi = SpiDriver::new(
        peripherals.spi3,
        peripherals.pins.gpio25,
        peripherals.pins.gpio32,
        Some(peripherals.pins.gpio39),
        &bus_cfg,
    )?;
    // TODO: hand `touch_spi` to your FT6x06 / ADS7846 / whatever driver.

    // keep the task alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
