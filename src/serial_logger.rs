//! Implements a logger over serial
//!
//! This allows the dev to just use the log crate instead of worrying about
//! complicated USB implementations.

use core::fmt::Write;
use log::{Level, Log};

use crate::hardware::Hardware;

/// Implements a logger for the log crate
///
/// This implementation is only useful with a usb manager class that supports
/// core::write_str
pub struct SerialLogger {}

impl SerialLogger {
    pub fn new() -> SerialLogger {
        SerialLogger {}
    }

    /// Writes the color escape code for this log level
    fn write_coloring(&self, level: &Level) -> () {
        let hardware = Hardware::get().unwrap();

        hardware
            .usb
            .write_str(match level {
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
        let hardware = Hardware::get().unwrap();

        let level = record.level();

        self.write_coloring(&level);

        // Message
        let args = record.args().clone();
        hardware.usb.write_fmt(args).unwrap();

        // Affix
        hardware.usb.write_str("\x1b[0m\r\n").unwrap();
    }

    fn flush(&self) {}
}
