#![no_std]
#![no_main]

#[link_section = ".boot2"]
#[no_mangle]
#[used]
static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

mod usb_manager;
use core::fmt::Write;

use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

use crate::usb_manager::UsbManager;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    entry, pac,
    pac::interrupt,
    sio::Sio,
    usb::UsbBus,
    watchdog::Watchdog,
};
use usb_device;
use usb_device::bus::UsbBusAllocator;

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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // rp2040_hal::usb::UsbBus::

    let usb = unsafe {
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
        USB_MANAGER.as_mut().unwrap()
    };

    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut pin1 = pins.gpio0.into_push_pull_output();

    let mut number = 0;

    loop {
        usb.write_fmt(format_args!("Hewwo, number {number}!\r"))
            .unwrap();

        pin1.set_high().unwrap();
        delay.delay_ms(1000);

        pin1.set_low().unwrap();
        number += 1;
        delay.delay_ms(1000);
    }
}
