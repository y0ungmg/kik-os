// PS/2 Keyboard driver (basic)

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

pub fn init() {
    // Basic initialization - enable keyboard
    // In a real implementation, we'd set up the controller properly
}

pub fn read_scancode() -> Option<u8> {
    unsafe {
        // Check if data is available
        let status = inb(KEYBOARD_STATUS_PORT);
        if status & 0x01 != 0 {
            Some(inb(KEYBOARD_DATA_PORT))
        } else {
            None
        }
    }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            out("al") value,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}