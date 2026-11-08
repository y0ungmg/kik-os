// KiK OS Init Process
// The first user-space process

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Print welcome message
    loop {
        // In a real implementation, this would:
        // 1. Initialize system services
        // 2. Start the shell
        // 3. Wait for user input
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}