#![no_std]
#![no_main]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

mod hardware;
mod serial_logger;
mod usb_manager;

use embedded_hal::digital::v2::OutputPin;
use hardware::Hardware;
use log::{debug, error, info, trace, warn};
use panic_reset as _;
use rp2040_hal::{
    entry,
    gpio::{bank0::Gpio0, Output, Pin, PushPull},
};
use serial_logger::SerialLogger;

static mut PIN1: Option<Pin<Gpio0, Output<PushPull>>> = None;

#[entry]
fn main() -> ! {
    let crystal_frequency = 12_000_000;
    Hardware::init(crystal_frequency);
    let hardware = Hardware::get().unwrap();

    SerialLogger::init(log::LevelFilter::Trace);

    // Start program logic
    let mut number = 0;

    loop {
        info!("Number: {number}");

        unsafe {
            PIN1.as_mut().unwrap().set_high().unwrap();
        };
        hardware.delay.delay_ms(1000);

        error!("Error");
        warn!("Warning");
        info!("Info");
        debug!("Debug");
        trace!("Trace");
        number += 1;

        unsafe {
            PIN1.as_mut().unwrap().set_low().unwrap();
        };
        hardware.delay.delay_ms(1000);
    }
}
