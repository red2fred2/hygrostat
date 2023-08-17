//! Handles most low level hardware abstraction

use cortex_m::delay::Delay;
use rp2040_hal::{clocks::init_clocks_and_plls, pac, usb::UsbBus, Clock, Sio, Watchdog};
use usb_device::class_prelude::UsbBusAllocator;

use crate::{usb_manager::UsbManager, PIN1, USB_BUS, USB_MANAGER};

pub struct Hardware {
    pub delay: Delay,
}

impl Hardware {
    /// Initialize RP2040 hardware
    pub fn init(crystal_frequency: u32) -> Hardware {
        let mut pac = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio = Sio::new(pac.SIO);

        let clocks = init_clocks_and_plls(
            crystal_frequency,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let delay;

        unsafe {
            delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

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

        // Pin setup
        unsafe { PIN1 = Some(pins.gpio0.into_push_pull_output()) };

        Hardware { delay }
    }
}
