// Kernel binary entry point
// This is linked with the kernel library

#![no_std]
#![no_main]

// Re-export kernel modules
mod serial;
mod gdt;
mod idt;
mod pmm;
mod heap;
mod timer;
mod keyboard;
mod framebuffer;
mod vmm;
mod scheduler;
mod syscalls;
mod ipc;
mod log;
mod pci;
mod kikfs;
mod net;
mod audio;
mod usb;
mod power;
mod users;
mod context_switch;

use core::panic::PanicInfo;

#[repr(C)]
pub struct BootInfo {
    pub magic: u64,
    pub kernel_load_addr: u64,
    pub kernel_size: u64,
    pub memory_map_addr: u64,
    pub memory_map_count: u64,
    pub framebuffer_addr: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_pitch: u32,
    pub framebuffer_bpp: u8,
    pub boot_mode: u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(_boot_info: *const BootInfo) -> ! {
    serial::init();
    serial::print("KiK OS kernel starting...\n");
    serial::print("Hello from KiK OS!\n");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial::print("KERNEL PANIC: ");
    if let Some(location) = info.location() {
        serial::print(location.file());
    }
    serial::print("\n");
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}