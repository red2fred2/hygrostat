#![no_std]
#![no_main]

#[link_section = ".boot2"]
#[no_mangle]
#[used]
static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

mod serial_logger;
mod usb_manager;

use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;
use log::{debug, error, info, trace, warn};
use panic_reset as _;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    entry, pac,
    pac::interrupt,
    sio::Sio,
    usb::UsbBus,
    watchdog::Watchdog,
};
use serial_logger::SerialLogger;
use usb_device;
use usb_device::bus::UsbBusAllocator;

use crate::usb_manager::UsbManager;

static mut DELAY: Option<Delay> = None;
static mut LOGGER: Option<SerialLogger> = None;
static LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;
static mut USB_BUS: Option<UsbBusAllocator<rp2040_hal::usb::UsbBus>> = None;
static mut USB_MANAGER: Option<UsbManager> = None;

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    match USB_MANAGER.as_mut() {
        Some(manager) => manager.interrupt(),
        None => (),
    };
}

#[entry]
fn main() -> ! {
    // Set up hardware
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    unsafe {
        DELAY = Some(cortex_m::delay::Delay::new(
            core.SYST,
            clocks.system_clock.freq().to_Hz(),
        ));

        USB_BUS = Some(UsbBusAllocator::new(UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        )));

        USB_MANAGER = Some(UsbManager::new(USB_BUS.as_ref().unwrap()));
        // Enable the USB interrupt
        pac::NVIC::unmask(rp2040_hal::pac::Interrupt::USBCTRL_IRQ);
    };

    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pin1 = pins.gpio0.into_push_pull_output();

    // Set up logging
    unsafe {
        LOGGER = Some(SerialLogger::new());
        log::set_logger_racy(LOGGER.as_ref().unwrap()).unwrap();
        log::set_max_level_racy(LOG_LEVEL);
    }

    // Start program logic
    let mut number = 0;

    loop {
        info!("Number: {number}");
        pin1.set_high().unwrap();
        unsafe { DELAY.as_mut().unwrap().delay_ms(1000) };

        error!("Error");
        warn!("Warning");
        info!("Info");
        debug!("Debug");
        trace!("Trace");
        pin1.set_low().unwrap();
        number += 1;
        unsafe { DELAY.as_mut().unwrap().delay_ms(1000) };
    }
}
