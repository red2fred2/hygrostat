use rp2040_hal as hal;
use usb_device;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::SerialPort;

pub struct UsbManager {
    device: UsbDevice<'static, hal::usb::UsbBus>,
    serial: SerialPort<'static, hal::usb::UsbBus>,
}

impl UsbManager {
    pub fn new(usb_bus: &'static UsbBusAllocator<hal::usb::UsbBus>) -> Self {
        let serial = usbd_serial::SerialPort::new(usb_bus);

        let device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x0000, 0x000b))
            .manufacturer("Polson")
            .product("Hygrostat")
            .serial_number("01")
            .device_class(2)
            .device_protocol(1)
            .build();

        UsbManager { device, serial }
    }

    pub unsafe fn interrupt(&mut self) {
        if self.device.poll(&mut [&mut self.serial]) {
            let mut data: [u8; 256] = [0x00; 256];

            let _ = self.serial.read(&mut data);
        }
    }
}

impl core::fmt::Write for UsbManager {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.serial.write(s.as_bytes()).unwrap();

        Ok(())
    }
}
