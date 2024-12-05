#![no_std]
#![no_main]

use dht11::Dht11;
use esp_backtrace as _;
use esp_println::println;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::*,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use esp_hal::xtensa_lx::timer::delay;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::max(system.clock_control).freeze();

    // Set GPIO4 as an output, and set its state high initially.
    let mut io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio4, Level::High);

    let pin27 = io.pins.gpio27;

    // let output_pin = Output::new(pin27, Level::Low);
    // let input_pin = Input::new(pin27, Pull::Up);
    let mut dht11 = Dht11::new(pin27);

    // let dht11_pin = AnyPin::new(io.pins.gpio27);
    // let mut dht11 = Dht11::new();

    led.set_high();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let delay = Delay::new(&clocks);
    println!("Starting DHT11 sensor reading...");
    loop {
        delay.delay_millis(1000);
        led.toggle();

        // let mut dht11_delay = delay.delay_millis(2000);

        // Need to figure out perform measurement()
        match dht11.perform_measurement() {
            Ok(measurement) => {
                println!("Temperature: {}°C", measurement.temperature as f32 / 10.0);
                println!("Humidity: {}%", measurement.humidity as f32 / 100.0);
            }
            Err(e) => println!("Measurement error: {:?}", e),
        }

        delay.delay_nanos(2000);
    }
}