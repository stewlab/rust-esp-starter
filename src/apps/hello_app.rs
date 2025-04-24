//! # hello_app
//! A simple example for ESP32-2432S028 that prints “hello” five times then halts.
//!  

use esp_idf_sys as _; // ...

/// Runs the hello_app:  
/// 1. Initializes the logger  
/// 2. Prints “hello” five times  
/// 3. Signals completion and halts.
pub fn run() {
    // Initialize the logger so println! outputs appear on the serial console
    esp_idf_svc::log::EspLogger::initialize_default();

    // Print "hello" five times
    for _ in 0..5 {
        println!("hello");
    }

    // Indicate completion
    println!("Done printing. Halting program.");

    // Halt here forever
    loop {
        // Prevent optimizers from removing the loop
        std::thread::sleep(core::time::Duration::from_millis(1));
    }
}
