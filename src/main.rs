#![no_std]
#![no_main]

use dht11::Dht11;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output},
    prelude::*,
};
use esp_hal::gpio::Input;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    // Set GPIO0 as an output, and set its state high initially.
    let mut led = Output::new(peripherals.GPIO4, Level::High);

    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(3000);
    }
}