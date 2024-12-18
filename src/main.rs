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
    // Initialize the ESP hardware abstraction layer (HAL) and get access to peripherals.
    // This also allows configuring things like the CPU clock speed.
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    // Configure GPIO4 as an output pin and set it to high level initially.
    // This pin can be used to drive an LED or another output device.
    let mut led = Output::new(peripherals.GPIO4, Level::High);

    // Create a flexible GPIO pin from GPIO0.
    // The DHT11 sensor requires a single data pin that can act as input and output.
    let dht11_pin = Flex::new(peripherals.GPIO0);

    // Instantiate a Dht11 sensor using the configured pin.
    let mut dht11 = Dht11::new(dht11_pin);

    // Create a Delay instance for timing operations.
    // The DHT11 communication protocol is timing-based, so accurate delays are needed.
    let delay = Delay::new();
    let mut dht11_delay = Delay::new();

    loop {
        // Toggle the LED state to indicate that the code is running.
        led.toggle();
        // Wait for 3 seconds before reading the sensor again.
        delay.delay_millis(3000);

        // Attempt to read temperature and humidity data from the DHT11 sensor.
        match dht11.perform_measurement(&mut dht11_delay) {
            Ok(sensor_data) => {
                // If successful, print the temperature and humidity values.
                esp_println::println!(
                    "Temperature: {}°C, Humidity: {}%",
                    sensor_data.temperature,
                    sensor_data.humidity
                );
            }
            Err(e) => {
                // If there was an error during measurement, print the error.
                esp_println::println!("Error reading from DHT11: {:?}", e);
            }
        }
    }
}