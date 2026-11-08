// Serial port driver (COM1) for debugging output
use core::fmt::Write;

const COM1: u16 = 0x3F8;

pub fn init() {
    unsafe {
        // Disable interrupts
        outb(COM1 + 1, 0x00);
        // Enable DLAB
        outb(COM1 + 3, 0x80);
        // Set divisor to 3 (38400 baud)
        outb(COM1 + 0, 0x03);
        outb(COM1 + 1, 0x00);
        // 8 bits, no parity, 1 stop bit
        outb(COM1 + 3, 0x03);
        // Enable FIFO
        outb(COM1 + 2, 0xC7);
        // IRQs enabled, RTS/DSR set
        outb(COM1 + 4, 0x0B);
    }
}

pub fn print(s: &str) {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        write_byte(bytes[i]);
        i += 1;
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        // Wait for transmit buffer to be empty
        while inb(COM1 + 5) & 0x20 == 0 {}
        outb(COM1, byte);
    }
}

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print(s);
        Ok(())
    }
}

pub struct SerialWriter;

unsafe fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    value
}