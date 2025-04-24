#![cfg(feature = "graphics-support")]

//! # rtos_shell_app
//! Interactive RTOS shell example on ESP32 using FreeRTOS.
//!  

use anyhow::{anyhow, Result};
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, Output, PinDriver},
    peripherals::Peripherals,
    prelude::*,
    spi::{Dma, SpiConfig, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};
use esp_idf_svc::{eventloop::EspSystemEventLoop, log::EspLogger, nvs::EspDefaultNvsPartition};
use esp_idf_sys as _; // Keeps `binstart` linkage
use log::*;
use st7789::{Orientation, ST7789};
use std::{
    collections::VecDeque,
    fs,
    io::{stdin, BufRead},
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

// Display parameters
const LCD_WIDTH: u16 = 240;
const LCD_HEIGHT: u16 = 320;
const SPI_BAUDRATE_HZ: u32 = 40_000_000;

// Shell layout
const MAX_SHELL_LOG_LINES: usize = 18;
const SHELL_START_Y: i32 = 15;
const SHELL_LINE_HEIGHT: i32 = 15;

// SPIFFS mount point (unused stub)
const FS_MOUNT_POINT: &str = "/spiffs";

// Shared shell state
#[derive(Debug)]
struct SharedState {
    shell_log: VecDeque<String>,
    needs_redraw: bool,
}

impl SharedState {
    fn new() -> Self {
        Self {
            shell_log: VecDeque::with_capacity(MAX_SHELL_LOG_LINES),
            needs_redraw: true,
        }
    }

    fn add_shell_message(&mut self, msg: String) {
        let max_chars = (LCD_WIDTH / 6) as usize - 2;
        let mut line = String::new();
        for word in msg.split_whitespace() {
            if line.is_empty() {
                line.push_str(word);
            } else if line.len() + 1 + word.len() <= max_chars {
                line.push(' ');
                line.push_str(word);
            } else {
                if self.shell_log.len() >= MAX_SHELL_LOG_LINES {
                    self.shell_log.pop_front();
                }
                self.shell_log.push_back(line.clone());
                line.clear();
                line.push_str(word);
            }
        }
        if !line.is_empty() {
            if self.shell_log.len() >= MAX_SHELL_LOG_LINES {
                self.shell_log.pop_front();
            }
            self.shell_log.push_back(line);
        }
        self.needs_redraw = true;
    }
}

// Map ST7789 errors
fn map_st7789_error<E: core::fmt::Debug>(err: st7789::Error<E>) -> anyhow::Error {
    anyhow!("ST7789 error: {:?}", err)
}

// Stub filesystem init (no-op)
fn init_fs() -> Result<&'static Path> {
    info!("Skipping SPIFFS init (stub)");
    Ok(Path::new(FS_MOUNT_POINT))
}

/// Runs the RTOS shell:  
/// starts FreeRTOS tasks for command handling and REPL.
pub fn run() -> Result<()> {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();
    let _evtloop = EspSystemEventLoop::take()?;
    let _nvs = EspDefaultNvsPartition::take()?;

    info!("Starting RTOS Shell App");
    let _ = init_fs().unwrap_or_else(|e| {
        error!("SPIFFS init failed: {}", e);
        Path::new("/")
    });

    let peripherals = Peripherals::take()?;

    // SPI & control pins
    let sclk = peripherals.pins.gpio14;
    let mosi = peripherals.pins.gpio13;
    let cs = peripherals.pins.gpio15;
    let dc = peripherals.pins.gpio2;
    let rst = peripherals.pins.gpio4;
    let bl = peripherals.pins.gpio21;

    let mut backlight = PinDriver::output(bl)?;
    let rst_drv = PinDriver::output(rst)?;
    let dc_drv = PinDriver::output(dc)?;

    // SPI setup
    let spi_drv = SpiDriver::new(
        peripherals.spi2,
        sclk,
        mosi,
        None::<AnyIOPin>,
        &SpiDriverConfig::new().dma(Dma::Disabled),
    )?;
    let spi_dev = SpiDeviceDriver::new(
        spi_drv,
        Some(cs),
        &SpiConfig::new()
            .baudrate(SPI_BAUDRATE_HZ.Hz())
            .write_only(true),
    )?;
    let iface = SPIInterfaceNoCS::new(spi_dev, dc_drv);

    // Display init
    let mut delay = FreeRtos;
    let mut disp: ST7789<_, _, PinDriver<'static, AnyIOPin, Output>> =
        ST7789::new(iface, Some(rst_drv), None, LCD_WIDTH, LCD_HEIGHT);
    disp.init(&mut delay).map_err(map_st7789_error)?;
    disp.set_orientation(Orientation::PortraitSwapped)
        .map_err(map_st7789_error)?;
    disp.clear(Rgb565::BLACK).map_err(map_st7789_error)?;
    backlight.set_high()?;

    // Shared state & display handle
    let shared = Arc::new(Mutex::new(SharedState::new()));
    let display = Arc::new(Mutex::new(disp));

    // Shell task
    {
        let shared = Arc::clone(&shared);
        thread::Builder::new().stack_size(4096).spawn(move || {
            let stdin = stdin();
            let mut reader = stdin.lock();
            let mut buf = String::new();

            shared
                .lock()
                .unwrap()
                .add_shell_message("Type 'help' for commands.".into());
            while reader.read_line(&mut buf).ok().filter(|&n| n > 0).is_some() {
                let input = buf.trim().to_string();
                buf.clear();
                if input.is_empty() {
                    continue;
                }

                let mut st = shared.lock().unwrap();
                st.add_shell_message(format!("> {}", input));
                match input.split_whitespace().next() {
                    Some("help") => st.add_shell_message("help, info, clear, ls [path]".into()),
                    Some("info") => {
                        let idf = unsafe {
                            let vp = esp_idf_sys::esp_get_idf_version();
                            if vp.is_null() {
                                "unknown"
                            } else {
                                std::ffi::CStr::from_ptr(vp).to_str().unwrap_or("<??>")
                            }
                        };
                        st.add_shell_message(format!("IDF: {}", idf));
                        let free = unsafe { esp_idf_sys::esp_get_free_heap_size() };
                        st.add_shell_message(format!("Heap: {} bytes", free));
                    }
                    Some("clear") => {
                        let mut s = shared.lock().unwrap();
                        s.shell_log.clear();
                        s.add_shell_message("Cleared".into());
                    }
                    Some("ls") => {
                        let path = input
                            .split_whitespace()
                            .nth(1)
                            .map(|p| format!("{}/{}", FS_MOUNT_POINT, p))
                            .unwrap_or_else(|| FS_MOUNT_POINT.into());
                        let mut s = shared.lock().unwrap();
                        s.add_shell_message(format!("Listing {}", path));
                        match fs::read_dir(&path) {
                            Ok(entries) => {
                                let mut any = false;
                                for e in entries.flatten() {
                                    let nm = e.file_name().into_string().unwrap_or("?".into());
                                    let meta = e.metadata().ok();
                                    let sz = meta
                                        .map(|m| {
                                            if m.is_dir() {
                                                "[DIR]".into()
                                            } else {
                                                format!("{}B", m.len())
                                            }
                                        })
                                        .unwrap_or_else(|| "?".into());
                                    s.add_shell_message(format!("- {} ({})", nm, sz));
                                    any = true;
                                }
                                if !any {
                                    s.add_shell_message("(empty)".into());
                                }
                            }
                            Err(e) => shared
                                .lock()
                                .unwrap()
                                .add_shell_message(format!("ls error: {}", e)),
                        }
                    }
                    Some(cmd) => st.add_shell_message(format!("Unknown: {}", cmd)),
                    None => {}
                }
            }
        })?;
    }

    // Display refresh loop
    loop {
        let mut s = shared.lock().unwrap();
        let redraw = s.needs_redraw;
        s.needs_redraw = false;
        drop(s);

        if redraw {
            let s = shared.lock().unwrap();
            let mut d = display.lock().unwrap();
            d.clear(Rgb565::BLACK).map_err(map_st7789_error)?;
            let style = MonoTextStyle::new(&FONT_6X10, Rgb565::CSS_LIGHT_BLUE);
            let mut y = SHELL_START_Y;
            for line in &s.shell_log {
                if y < (LCD_HEIGHT as i32 - SHELL_LINE_HEIGHT) {
                    Text::new(line, Point::new(5, y), style)
                        .draw(&mut *d)
                        .map_err(map_st7789_error)?;
                    y += SHELL_LINE_HEIGHT;
                }
            }
        }
        FreeRtos::delay_ms(50);
    }
}
