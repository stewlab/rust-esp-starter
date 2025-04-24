#![cfg(feature = "graphics-support")]

//! # mud_game_app
//! A simple text-based MUD game running on ESP32 via serial.
//!  

use anyhow::{anyhow, Result};
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use esp_idf_hal::prelude::*;
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, PinDriver},
    peripherals::Peripherals,
    spi::{SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _;
use st7789::{Orientation, ST7789};

// Display constants
const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;
const SPI_BAUDRATE_HZ: u32 = 40_000_000;

fn map_st7789_error<E: core::fmt::Debug>(err: st7789::Error<E>) -> anyhow::Error {
    anyhow!("ST7789 driver error: {:?}", err)
}

struct GameState {
    player_health: i32,
    player_level: i32,
    current_room: usize,
    messages: Vec<&'static str>,
    dirty: bool, // New flag to track changes
}

impl GameState {
    fn new() -> Self {
        Self {
            player_health: 100,
            player_level: 1,
            current_room: 0,
            messages: vec![
                "You enter the ancient crypt...",
                "The door slams shut behind you!",
                "You hear strange noises...",
            ],
            dirty: true, // Start with dirty flag set
        }
    }

    fn add_message(&mut self, msg: &'static str) {
        self.messages.push(msg);
        if self.messages.len() > 5 {
            self.messages.remove(0);
        }
        self.dirty = true; // Mark state as changed
    }

    fn simulate_turn(&mut self) {
        let prev_health = self.player_health;
        self.player_health = (self.player_health - 1).max(0);
        if self.player_health % 20 == 0 {
            self.add_message("You feel a cold presence...");
        }
        if prev_health != self.player_health {
            self.dirty = true; // Mark state as changed
        }
    }
}

/// Runs the MUD game shell:  
/// listens on UART, processes commands, and responds.
pub fn run() -> Result<()> {
    EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;

    // Display initialization (same as original example)
    let sclk = peripherals.pins.gpio14;
    let mosi = peripherals.pins.gpio13;
    let cs_pin = peripherals.pins.gpio15;
    let dc_pin = peripherals.pins.gpio2;
    let rst_pin = peripherals.pins.gpio0;
    let bl_pin = peripherals.pins.gpio21;

    let mut backlight = PinDriver::output(bl_pin)?;
    backlight.set_high()?;
    let rst = PinDriver::output(rst_pin)?;
    let dc = PinDriver::output(dc_pin)?;

    let spi_driver = SpiDriver::new(
        peripherals.spi2,
        sclk,
        mosi,
        None::<AnyIOPin>,
        &SpiDriverConfig::new(),
    )?;
    let spi_cfg = SpiConfig::new()
        .baudrate(SPI_BAUDRATE_HZ.Hz())
        .write_only(true);
    let spi_device = SpiDeviceDriver::new(spi_driver, Some(cs_pin), &spi_cfg)?;
    let di = SPIInterfaceNoCS::new(spi_device, dc);
    let mut delay = FreeRtos;
    let mut display = ST7789::new(di, Some(rst), Some(backlight), LCD_WIDTH, LCD_HEIGHT);
    display.init(&mut delay).map_err(map_st7789_error)?;
    display
        .set_orientation(Orientation::PortraitSwapped)
        .map_err(map_st7789_error)?;

    // Game state and styles
    let mut game_state = GameState::new();
    let header_style = MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_RED);
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_WHITE);
    let alert_style = MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_YELLOW);

    // Main game loop
    let mut counter = 0;
    loop {
        // Game logic update
        if counter % 30 == 0 {
            game_state.simulate_turn();
        }

        // Only redraw when there are changes
        if game_state.dirty {
            // Batch all drawing operations
            display.clear(Rgb565::BLACK).map_err(map_st7789_error)?;

            // Draw player stats
            Text::new(
                &format!("Health: {}", game_state.player_health),
                Point::new(10, 10),
                text_style,
            )
            .draw(&mut display)
            .map_err(map_st7789_error)?;

            Text::new(
                &format!("Level:  {}", game_state.player_level),
                Point::new(10, 25),
                text_style,
            )
            .draw(&mut display)
            .map_err(map_st7789_error)?;

            // Draw room description
            Text::new("Dungeon Level 1", Point::new(10, 50), header_style)
                .draw(&mut display)
                .map_err(map_st7789_error)?;

            Text::new(
                "You stand in a dark chamber",
                Point::new(10, 65),
                text_style,
            )
            .draw(&mut display)
            .map_err(map_st7789_error)?;

            Text::new("with ancient carvings on", Point::new(10, 80), text_style)
                .draw(&mut display)
                .map_err(map_st7789_error)?;

            Text::new("the walls. Three exits.", Point::new(10, 95), text_style)
                .draw(&mut display)
                .map_err(map_st7789_error)?;

            // Draw message log
            Text::new("Messages:", Point::new(10, 120), header_style)
                .draw(&mut display)
                .map_err(map_st7789_error)?;

            for (i, msg) in game_state.messages.iter().rev().take(5).enumerate() {
                Text::new(msg, Point::new(10, 135 + (i as i32 * 15)), alert_style)
                    .draw(&mut display)
                    .map_err(map_st7789_error)?;
            }

            // Game logic update
            if counter % 30 == 0 {
                game_state.simulate_turn();
            }

            game_state.dirty = false;
        }

        counter += 1;
        FreeRtos::delay_ms(100);
    }
}
