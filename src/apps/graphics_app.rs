#![cfg(feature = "graphics-support")]

//! # graphics_app
//! Renders basic shapes using embedded-graphics on an ESP32.
//!  

use anyhow::{anyhow, Result};
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{
    // delay::Ets, // Remove Ets
    delay::FreeRtos, // <-- Use FreeRtos delay
    gpio::{AnyIOPin, AnyOutputPin, Output, PinDriver},
    peripherals::Peripherals,
    prelude::*,
    spi::{
        config::{Config as SpiConfig, Mode as SpiMode},
        Dma, SpiDeviceDriver, SpiDriver,
    },
};
// embedded_hal::delay::DelayNs is implicitly implemented by FreeRtos delay,
// but let's import it explicitly for clarity if needed by other code.
use embedded_hal::delay::DelayNs;
use st7789::{Orientation, ST7789};

// --- Configuration Constants ---
const LCD_PIN_NUM_SCLK: u32 = 18;
const LCD_PIN_NUM_MOSI: u32 = 19;
const LCD_PIN_NUM_MISO: Option<u32> = None;
const LCD_PIN_NUM_CS: u32 = 5;
const LCD_PIN_NUM_DC: u32 = 16;
const LCD_PIN_NUM_RST: u32 = 23;
const LCD_PIN_NUM_BCKL: u32 = 4;
const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;
const SPI_BAUDRATE_HZ: u32 = 40 * 1_000_000;
const SPI_DMA_BUFFER_SIZE: usize = 4096;
const SPI_DMA_CONFIG: Dma = Dma::Auto(SPI_DMA_BUFFER_SIZE);

fn map_st7789_error<E: core::fmt::Debug>(err: st7789::Error<E>) -> anyhow::Error {
    anyhow!("ST7789 driver error: {:?}", err)
}

/// Runs the graphics demo:  
/// draws primitives (lines, rectangles, text) on the connected display.
pub fn run() -> Result<()> {
    log::info!("Starting graphics application...");

    // --- Peripheral Initialization ---
    log::info!("Taking peripherals...");
    let peripherals = Peripherals::take()?;

    // --- GPIO Pin Setup ---
    log::info!("Configuring GPIO pins...");
    let sclk = peripherals.pins.gpio14;
    let mosi = peripherals.pins.gpio13;
    let miso: Option<AnyIOPin> = LCD_PIN_NUM_MISO
        .map(|num| match num {
            _ => panic!("MISO pin number {} not handled", num),
        })
        .or_else(|| None::<core::convert::Infallible>.map(|_| -> AnyIOPin { unreachable!() }));
    let cs_pin = peripherals.pins.gpio15;
    let dc_pin = peripherals.pins.gpio2;
    let rst_pin = peripherals.pins.gpio0;
    let bckl_pin = peripherals.pins.gpio21;

    // --- Backlight Control ---
    let mut backlight = PinDriver::output(bckl_pin)?;
    backlight.set_high()?;
    log::info!("Backlight pin configured and turned ON");

    // --- Reset Pin ---
    let rst = PinDriver::output(rst_pin)?;
    log::info!("Reset pin configured");

    // --- Data/Command Pin ---
    let dc = PinDriver::output(dc_pin)?;
    log::info!("Data/Command pin configured");

    // --- SPI Configuration ---
    // ... (using DMA) ...
    log::info!(
        "Configuring SPI driver with DMA enabled (Buffer: {} bytes)...",
        SPI_DMA_BUFFER_SIZE
    );
    let spi_driver = SpiDriver::new(
        peripherals.spi2,
        sclk,
        mosi,
        miso,
        &esp_idf_hal::spi::SpiDriverConfig::new().dma(SPI_DMA_CONFIG),
    )?;
    log::info!("SPI driver created");
    let spi_config = SpiConfig::new()
        .baudrate(SPI_BAUDRATE_HZ.Hz())
        .write_only(true)
        .data_mode(SpiMode {
            polarity: embedded_hal::spi::Polarity::IdleLow,
            phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
        });
    log::info!(
        "SPI config created (Baudrate: {} Hz, WriteOnly: true)",
        SPI_BAUDRATE_HZ
    );
    let cs_any_output: AnyOutputPin = cs_pin.into();
    log::info!("CS pin converted to AnyOutputPin");
    let spi_device = SpiDeviceDriver::new(&spi_driver, Some(cs_any_output), &spi_config)?;
    log::info!("SPI device driver created successfully");

    // --- Display Interface ---
    let di = SPIInterfaceNoCS::new(spi_device, dc);
    log::info!("Display interface created");

    // --- Initialize Display ---
    // Use FreeRtos delay provider now
    // It implements DelayNs needed by init()
    let mut delay = FreeRtos;

    let bl_pin: Option<PinDriver<'static, AnyOutputPin, Output>> = None;
    let mut display = ST7789::new(di, Some(rst), bl_pin, LCD_WIDTH, LCD_HEIGHT);

    log::info!("Initializing ST7789 display...");
    // Pass FreeRtos delay to init
    display.init(&mut delay).map_err(map_st7789_error)?; // <--- delay is now FreeRtos
    log::info!("ST7789 driver initialized successfully");

    // --- Set the desired orientation ---
    display
        .set_orientation(Orientation::PortraitSwapped) // Use PortraitFlipped for 180 degrees
        .map_err(map_st7789_error)?;
    log::info!("Display orientation set to PortraitFlipped (180 degrees)"); // Updated log message

    // --- Drawing Example ---
    log::info!("Clearing display to BLACK...");
    display.clear(Rgb565::BLACK).map_err(map_st7789_error)?;
    log::info!("Display cleared");

    log::info!("Drawing text...");
    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_LIME);
    Text::new("Hello Rust + ESP32!", Point::new(20, 30), text_style)
        .draw(&mut display)
        .map_err(map_st7789_error)?;

    let text_style_2 = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_YELLOW);
    Text::new("Graphics OK", Point::new(20, 60), text_style_2)
        .draw(&mut display)
        .map_err(map_st7789_error)?;
    log::info!("Text drawn on display");

    log::info!("Graphics app finished drawing. Entering loop.");

    // --- Keep Running ---
    loop {
        // Use FreeRtos::delay_ms directly (it's a static method)
        // This call yields control to the scheduler.
        FreeRtos::delay_ms(1000);
    }
    // Ok(())
}
