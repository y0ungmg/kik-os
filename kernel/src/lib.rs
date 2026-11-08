#![no_std]
#![no_main]

use core::panic::PanicInfo;

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

// Boot info structure (must match bootloader's definition)
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
pub extern "C" fn _start(boot_info: *const BootInfo) -> ! {
    serial::init();
    serial::print("KiK OS kernel starting...\n");
    
    // Initialize GDT
    gdt::init();
    serial::print("GDT initialized.\n");
    
    // Initialize IDT
    idt::init();
    serial::print("IDT initialized.\n");
    
    // Initialize PMM (128MB default)
    pmm::init(128 * 1024 * 1024);
    serial::print("PMM initialized.\n");
    
    // Initialize heap
    heap::init();
    serial::print("Heap initialized.\n");
    
    // Initialize timer (100 Hz)
    timer::init(100);
    serial::print("Timer initialized.\n");
    
    // Initialize keyboard
    keyboard::init();
    serial::print("Keyboard initialized.\n");
    
    // Initialize VMM
    // vmm::init(); // Placeholder
    
    // Initialize scheduler
    scheduler::init();
    serial::print("Scheduler initialized.\n");
    
    if !boot_info.is_null() {
        let bi = unsafe { &*boot_info };
        serial::print("Boot info received.\n");
        if bi.boot_mode == 0 {
            serial::print("Boot mode: BIOS\n");
        } else {
            serial::print("Boot mode: UEFI\n");
        }
    }
    
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