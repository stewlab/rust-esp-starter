#![cfg(feature = "graphics-support")]

//! # rotating_cube_app
//! Displays a 3D rotating cube using embedded-graphics.
//!

use anyhow::{anyhow, Result};
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::Text,
};
use embedded_hal::spi::MODE_0;
use esp_idf_hal::prelude::*; // brings in FromValueType for Hz()
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, PinDriver},
    peripherals::Peripherals,
    spi::{SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _; // ensure ESP-IDF linkage
use st7789::{Orientation, ST7789};

// Display constants
const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;
const SPI_BAUDRATE_HZ: u32 = 40_000_000; // 40 MHz

fn map_st7789_error<E: core::fmt::Debug>(err: st7789::Error<E>) -> anyhow::Error {
    anyhow!("ST7789 driver error: {:?}", err)
}

/// Runs the rotating cube demo:  
/// draws and animates a wireframe cube.
pub fn run() -> Result<()> {
    EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let sclk = peripherals.pins.gpio14;
    let mosi = peripherals.pins.gpio13;
    let miso: Option<AnyIOPin> = None;
    let cs_pin = peripherals.pins.gpio15;
    let dc_pin = peripherals.pins.gpio2;
    let rst_pin = peripherals.pins.gpio0;
    let bl_pin = peripherals.pins.gpio21;

    let mut backlight = PinDriver::output(bl_pin)?;
    backlight.set_high()?;
    let rst = PinDriver::output(rst_pin)?;
    let dc = PinDriver::output(dc_pin)?;

    let spi_driver = SpiDriver::new(peripherals.spi2, sclk, mosi, miso, &SpiDriverConfig::new())?;
    let spi_cfg = SpiConfig::new()
        .baudrate(SPI_BAUDRATE_HZ.Hz())
        .write_only(true)
        .data_mode(MODE_0);
    let spi_device = SpiDeviceDriver::new(spi_driver, Some(cs_pin), &spi_cfg)?;
    let di = SPIInterfaceNoCS::new(spi_device, dc);
    let mut delay = FreeRtos;
    let mut display = ST7789::new(di, Some(rst), Some(backlight), LCD_WIDTH, LCD_HEIGHT);
    display.init(&mut delay).map_err(map_st7789_error)?;
    display
        .set_orientation(Orientation::PortraitSwapped)
        .map_err(map_st7789_error)?;

    // Static intro
    display.clear(Rgb565::BLACK).map_err(map_st7789_error)?;
    let style1 = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_LIME);
    let style2 = MonoTextStyle::new(&FONT_10X20, Rgb565::CSS_YELLOW);
    Text::new("Hello Rust + ESP32!", Point::new(20, 30), style1)
        .draw(&mut display)
        .map_err(map_st7789_error)?;
    Text::new("Graphics OK", Point::new(20, 60), style2)
        .draw(&mut display)
        .map_err(map_st7789_error)?;

    // Prepare cube demo
    const POINTS: [(f32, f32, f32); 8] = [
        (-1.0, -1.0, -1.0),
        (-1.0, -1.0, 1.0),
        (-1.0, 1.0, -1.0),
        (-1.0, 1.0, 1.0),
        (1.0, -1.0, -1.0),
        (1.0, -1.0, 1.0),
        (1.0, 1.0, -1.0),
        (1.0, 1.0, 1.0),
    ];
    const EDGES: [(usize, usize); 12] = [
        (0, 1),
        (0, 2),
        (0, 4),
        (1, 3),
        (1, 5),
        (2, 3),
        (2, 6),
        (3, 7),
        (4, 5),
        (4, 6),
        (5, 7),
        (6, 7),
    ];
    let cx = (LCD_WIDTH / 2) as f32;
    let cy = (LCD_HEIGHT / 2) as f32;
    let scale = 60.0;
    let dist = 3.0;
    let angle_step = 0.05;
    let y_mul = 0.7;
    let mut angle: f32 = 0.0;
    let style_line = PrimitiveStyle::with_stroke(Rgb565::WHITE, 1);
    let mut projected = [Point::new(0, 0); 8];

    loop {
        display.clear(Rgb565::BLACK).map_err(map_st7789_error)?;
        let sinx = angle.sin();
        let cosx = angle.cos();
        let siny = (angle * y_mul).sin();
        let cosy = (angle * y_mul).cos();

        for (i, &(x, y, z)) in POINTS.iter().enumerate() {
            let y2 = y * cosx - z * sinx;
            let z2 = y * sinx + z * cosx;
            let x3 = x * cosy + z2 * siny;
            let z3 = -x * siny + z2 * cosy;
            let depth = dist + z3;
            projected[i] = Point::new(
                ((x3 * (dist / depth)) * scale + cx) as i32,
                ((y2 * (dist / depth)) * scale + cy) as i32,
            );
        }

        for &(i, j) in &EDGES {
            Line::new(projected[i], projected[j])
                .into_styled(style_line)
                .draw(&mut display)
                .map_err(map_st7789_error)?;
        }

        angle += angle_step;
        FreeRtos::delay_ms(30); // shorter delay for higher FPS
    }
}
