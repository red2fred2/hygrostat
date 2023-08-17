#![no_std]
#![no_main]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

mod hardware;
mod pin_test;
mod serial_logger;
mod usb_manager;

use hardware::Hardware;
use log::{debug, error, info, trace, warn};
use panic_reset as _;
use rp2040_hal::entry;
use serial_logger::SerialLogger;

#[entry]
fn main() -> ! {
    let crystal_frequency = 12_000_000;
    Hardware::init(crystal_frequency);
    let hardware = Hardware::get().unwrap();

    SerialLogger::init(log::LevelFilter::Trace);

    loop {
        hardware.pins.set_high();
        hardware.delay.delay_ms(1000);

        error!("Error");
        warn!("Warning");
        info!("Info");
        debug!("Debug");
        trace!("Trace");
        hardware.pins.set_low();
        hardware.delay.delay_ms(1000);
    }
}
