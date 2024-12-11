#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output, Flex},
    i2c::{I2C, MasterPins, MasterConfig},
    peripherals::Peripherals,
    prelude::*,
};

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
    mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
    text::Text,
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};


#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO4, Level::High);

    let i2c_pins = MasterPins {
        sda: peripherals.GPIO22,
        scl: peripherals.GPIO21,
    };

    /// DHT11 Sensor needs
    // let dht11_pin = Flex::new(peripherals.GPIO0);
    // let mut dht11 = Dht11::new(dht11_pin);
    // let mut dht11_delay = Delay::new();

    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(1500);

        // Attempted to run .perform_measurement not getting any data back
        // could be possible due to the fact that the version of the dht11 crate. isn't working.
        // match dht11.perform_measurement(&mut dht11_delay) {
        //     Ok(sensor_data) => {
        //         esp_println::println!(
        //             "Temperature: {}°C, Humidity: {}%",
        //             sensor_data.temperature,
        //             sensor_data.humidity
        //         );
        //     }
        //     Err(e) => {
        //         esp_println::println!("Error reading from DHT11: {:?}", e);
        //     }
        // }
    }
}