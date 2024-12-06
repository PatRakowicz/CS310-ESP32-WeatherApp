#![no_std]
#![no_main]

use dht11::Dht11;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output, Flex},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO4, Level::High);
    let dht11_pin = Flex::new(peripherals.GPIO0);
    let mut dht11 = Dht11::new(dht11_pin);

    let delay = Delay::new();
    let mut dht11_delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(3000);

        match dht11.perform_measurement(&mut dht11_delay) {
            Ok(sensor_data) => {
                esp_println::println!(
                    "Temperature: {}°C, Humidity: {}%",
                    sensor_data.temperature,
                    sensor_data.humidity
                );
            }
            Err(e) => {
                esp_println::println!("Error reading from DHT11: {:?}", e);
            }
        }
    }
}