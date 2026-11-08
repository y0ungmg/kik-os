#![no_std]
#![no_main]

use uefi::prelude::*;
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Embed the kernel binary
static KERNEL: &[u8] = include_bytes!("../../../target/x86_64-unknown-none/debug/kik-kernel");

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    
    uefi::system::with_stdout(|stdout| {
        let _ = stdout.write_str("KiK Boot - UEFI\n");
        let _ = stdout.write_str("Loading KiK OS kernel...\n");
        let _ = stdout.write_str("Kernel size: ");
        // Print size (simplified)
        let _ = stdout.write_str(" bytes\n");
    });
    
    // In a real implementation, we'd:
    // 1. Parse ELF headers
    // 2. Load segments into memory
    // 3. Jump to entry point
    // For now, just hang
    loop {}
}