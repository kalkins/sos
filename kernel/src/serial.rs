use core::fmt;

use crate::arch::serial::SerialImpl;

pub struct SerialPort;

impl SerialPort {
    pub fn init() {
        SerialImpl::init();
    }

    pub fn send(byte: u8) {
        SerialImpl::send(byte);
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            SerialPort::send(byte);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SerialPort.write_fmt(args).unwrap();
}
