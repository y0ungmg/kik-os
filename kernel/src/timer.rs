// PIT (Programmable Interval Timer) driver

const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND: u16 = 0x43;

static mut TICKS: u64 = 0;

pub fn init(frequency: u32) {
    let divisor = 1193182 / frequency;
    
    unsafe {
        // Channel 0, lobyte/hibyte, rate generator
        outb(PIT_COMMAND, 0x36);
        outb(PIT_CHANNEL0, (divisor & 0xFF) as u8);
        outb(PIT_CHANNEL0, ((divisor >> 8) & 0xFF) as u8);
    }
}

pub fn get_ticks() -> u64 {
    unsafe { TICKS }
}

pub fn increment_ticks() {
    unsafe { TICKS += 1; }
}

unsafe fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}