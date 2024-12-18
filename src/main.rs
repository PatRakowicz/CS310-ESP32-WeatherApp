#![no_std]
#![no_main]

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
    let peripherals = esp_hal::init({
        let mut config = Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let mut led = Output::new(peripherals.GPIO4, Level::High);


    // Ic2 Display https://docs.rs/ssd1306/0.9.0/ssd1306/index.html

    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(
        interface,
        DisplaySize128x64,
        DisplayRotation::Rotate0,
    ).into_buffered_graphics_mode();

    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hi Amy!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    let mut x_pos = 0;
    let delay = Delay::new();

    loop {
        led.toggle();


        Rectangle::new(Point::new(0, 0), Size::new(128, 64))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
            .draw(&mut display)
            .unwrap();

        Text::with_baseline("Moving Text!", Point::new(x_pos, 32), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        let mut x_pos = 0;

        x_pos += 4;
        if x_pos > 128 {
            x_pos = -64;
        }
        delay.delay_millis(200);
    }
}
