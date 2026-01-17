use crate::arch::io::{inb, outb};

pub struct SerialImpl;

const PORT: u16 = 0x3F8;

impl SerialImpl {
    pub fn init() {
        unsafe {
            outb(PORT + 1, 0x00); // Disable interrupts
            outb(PORT + 3, 0x80); // Enable DLAB
            outb(PORT + 0, 0x03); // Set 38400 baud
            outb(PORT + 1, 0x00); // (hi byte)
            outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
            outb(PORT + 2, 0xC7); // Enable FIFO
            outb(PORT + 4, 0x08); // IRQs enabled, RTS/DSR set
        }
    }

    pub fn send(byte: u8) {
        unsafe {
            while (inb(PORT + 5) & 0x20) == 0 {}
            outb(PORT, byte);
        }
    }
}
