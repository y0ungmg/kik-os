// PIT (Programmable Interval Timer) driver

const PIT_CHANNEL0: u16 = 0x40;
const PIT_COMMAND: u16 = 0x43;

static mut TICKS: u64 = 0;

pub fn init(frequency: u32) {
    let divisor = 1193182 / frequency;
    
    unsafe {
        core::arch::asm!("out 0x43, al", in("al") 0x36u8, options(nomem, nostack));
        core::arch::asm!("out 0x40, al", in("al") (divisor & 0xFF) as u8, options(nomem, nostack));
    }
}

pub fn get_ticks() -> u64 {
    unsafe { TICKS }
}

pub fn increment_ticks() {
    unsafe { TICKS += 1; }
}

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack)
    );
}