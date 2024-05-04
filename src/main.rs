use pn532::{requests::SAMMode, spi::SPIInterface, Pn532, Request};
use pn532::IntoDuration; // trait for `ms()`, your HAL might have its own

use esp_idf_svc::hal::gpio::*;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::prelude::Peripherals;

// spi, cs and timer are structs implementing their respective embedded_hal traits

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let mut led = PinDriver::output(peripherals.pins.gpio2)?;

    loop {
        println!("high");
        led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);

        led.set_low()?;
        println!("low");
        FreeRtos::delay_ms(1000);
    }
}