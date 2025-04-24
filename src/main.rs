//! # rust-esp-starter
//! Entry-point for the Rust ESP32 starter template.
//!

mod apps;

/// Application entry point.  
/// By default runs the `hello_app`.  
fn main() {
    crate::apps::hello_app::run();
    // crate::apps::led_blinking_app::run();

    // #[cfg(feature = "display-support")]
    // crate::apps::display_backlight_app::run().unwrap();

    // #[cfg(feature = "graphics-support")]
    // crate::apps::graphics_app::run().unwrap();

    // #[cfg(feature = "graphics-support")]
    // crate::apps::rotating_cube_app::run().unwrap();

    // #[cfg(feature = "graphics-support")]
    // crate::apps::mud_game_app::run().unwrap();

    // #[cfg(feature = "graphics-support")]
    // crate::apps::rtos_shell_app::run().unwrap();
}
