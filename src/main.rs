//! This controls a hygrostat for a cheese cave.
//!
//! The complete set of files can be found on the [Github] repo.
//!
//!	To load new code to the device:
//!
//! 0. [Install] Rust.
//! 1. Run ```cargo run --release``` to generate a .uf2 file for flashing. This
//! can be found in the ```target > thumbv6m-none-eabi > release``` folder.
//! 2. Short the test pads on the board on the edge opposite the big capacitor.
//! 3. Plug the board into your computer with a USB cord.
//! 4. Stop shorting the test pads.
//! 5. The board should show up as a flash drive. Drag and drop the .uf2 file in.
//! 6. It will disconnect from the computer and reboot with the new code.
//!
//! [Install]: https://www.rust-lang.org/tools/install
//! [Github]: https://github.com/red2fred2/hygrostat/

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
