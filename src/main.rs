#![no_std]
#![no_main]

#[link_section = ".boot2"]
#[no_mangle]
#[used]
static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

use rp2040_hal::{
    // clocks::{init_clocks_and_plls, Clock},
    entry,
    pac,
    sio::Sio,
    // watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    // let core = pac::CorePeripherals::take().unwrap();
    // let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // let external_xtal_freq_hz = 12_000_000u32;
    // let clocks = init_clocks_and_plls(
    //     external_xtal_freq_hz,
    //     pac.XOSC,
    //     pac.CLOCKS,
    //     pac.PLL_SYS,
    //     pac.PLL_USB,
    //     &mut pac.RESETS,
    //     &mut watchdog,
    // )
    // .ok()
    // .unwrap();

    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pin2 = pins.gpio2.into_push_pull_output();
    let mut pin3 = pins.gpio3.into_push_pull_output();
    let mut pin4 = pins.gpio4.into_push_pull_output();
    let mut pin5 = pins.gpio5.into_push_pull_output();

    pin2.set_high().unwrap();
    pin3.set_high().unwrap();
    pin4.set_high().unwrap();
    pin5.set_high().unwrap();

    loop {}
}
