//! Implements a logger over serial
//!
//! This allows the dev to just use the log crate instead of worrying about
//! complicated USB implementations.

use core::fmt::Write;
use log::{Level, Log};

pub struct SerialLogger {}

impl SerialLogger {
    pub fn new() -> SerialLogger {
        SerialLogger {}
    }

    fn write_coloring(&self, level: &Level) -> () {
        let usb = unsafe { crate::USB_MANAGER.as_mut().unwrap() };

        usb.write_str(match level {
            Level::Error => "\x1b[31;1m",
            Level::Warn => "\x1b[33;1m",
            Level::Info => "\x1b[37m",
            Level::Debug => "\x1b[35m",
            Level::Trace => "\x1b[36m",
        })
        .unwrap()
    }
}

impl Log for SerialLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let usb = unsafe { crate::USB_MANAGER.as_mut().unwrap() };
        let level = record.level();

        self.write_coloring(&level);

        // Message
        let args = record.args().clone();
        usb.write_fmt(args).unwrap();

        // Affix
        usb.write_str("\x1b[0m\r\n").unwrap();
    }

    fn flush(&self) {}
}
