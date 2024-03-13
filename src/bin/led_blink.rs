use std::{thread, time::Duration};

use esp_idf_svc::hal::{gpio::PinDriver, prelude::Peripherals};

fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio4).unwrap();

    loop {
        led.set_high().unwrap();
        thread::sleep(Duration::from_millis(1000));

        led.set_low().unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
