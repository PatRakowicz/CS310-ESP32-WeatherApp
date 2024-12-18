#![no_std] // Do not use the Rust standard library
#![no_main] // Do not use the normal Rust entry point chain

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output, Flex},
    i2c::master::{I2c, Config as I2cConfig},
    peripherals::Peripherals,
    prelude::*,
    Config
};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    text::Text,
};
use embedded_graphics::text::Baseline;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

#[entry]
fn main() -> ! {
    // Initialize peripherals, setting up clocks and default configurations.
    // esp_hal::init returns a Peripherals struct with references to various hardware resources.
    let peripherals = esp_hal::init({
        let mut config = Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    // Configure GPIO4 as an output pin and set it initially high.
    // We will use this LED as a visual indicator or heartbeat LED.
    let mut led = Output::new(peripherals.GPIO4, Level::High);


    // Ic2 Display https://docs.rs/ssd1306/0.9.0/ssd1306/index.html
    // Set up the I2C interface for the OLED display.
    // SDA on GPIO21 and SCL on GPIO22 is common for ESP boards like the ESP32.
    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    // Create a display interface using the I2C interface just configured.
    let interface = I2CDisplayInterface::new(i2c);
    // Initialize the SSD1306 display driver in buffered graphics mode.
    // Using a 128x64 display with no rotation.
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();

    // Perform the hardware initialization routine for the SSD1306 display.
    display.init().unwrap();

    // Set up a text style using a 6x10 font and enabling the pixel (binary) color.
    // MonoTextStyleBuilder is from embedded-graphics and helps define text properties.
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    // Draw a "Hello world!" message at the top-left corner (0,0) of the display.
    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    // Draw a second line of text "Hi Amy!" at (0,16) to show multiple lines of text.
    Text::with_baseline("Hi Amy!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    // Initial x-position for a piece of text that will scroll across the screen.
    let mut x_pos = 0;

    // Create a new Delay instance to handle timing (delays in milliseconds).
    let delay = Delay::new();

    loop {
        // Toggle the LED state each iteration, giving a visible sign of running code.
        led.toggle();

        // Clear the entire display by drawing a filled rectangle of 'Off' color over it.
        Rectangle::new(Point::new(0, 0), Size::new(128, 64))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();

        // Draw text that will move horizontally across the screen.
        // The text baseline is set at Y=32, and the horizontal position will change each loop.
        Text::with_baseline("Moving Text!", Point::new(x_pos, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        // Move the text 4 pixels to the right each loop iteration.
        // After it goes beyond the display width (128), reset it back to -64
        // to create a smooth scrolling effect.
        x_pos += 4;
        if x_pos > 128 {
            x_pos = -64;
        }

        // Wait for 200ms before the next iteration.
        delay.delay_millis(200);
    }
}
