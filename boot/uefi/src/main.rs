#![no_std]
#![no_main]

use uefi::prelude::*;
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    
    uefi::system::with_stdout(|stdout| {
        let _ = stdout.write_str("KiK Boot - UEFI\n");
        let _ = stdout.write_str("Loading KiK OS kernel...\n");
    });
    
    // For now, just print and hang
    loop {}
}