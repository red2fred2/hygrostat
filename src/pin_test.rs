//! A placeholder pin test class

use embedded_hal::digital::v2::OutputPin;
use log::error;
use rp2040_hal::gpio::{bank0::Gpio0, Output, Pin, Pins, PushPull};

pub struct PinTest {
    pin: Pin<Gpio0, Output<PushPull>>,
}

impl PinTest {
    pub fn new(pins: Pins) -> Self {
        let pin = pins.gpio0.into_push_pull_output();

        Self { pin }
    }

    /// Sets the test pin high
    pub fn set_high(&mut self) {
        let result = self.pin.set_high();

        if result.is_err() {
            error!("Failed to set test pin high");
        }
    }

    /// Sets the test pin low
    pub fn set_low(&mut self) {
        let result = self.pin.set_low();

        if result.is_err() {
            error!("Failed to set test pin low");
        }
    }
}
