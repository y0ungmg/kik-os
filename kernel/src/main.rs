// Kernel binary entry point
#![no_std]
#![no_main]

mod serial;
mod gdt;
mod idt;
mod pmm;
mod heap;
mod timer;
mod vmm;

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
    
    gdt::init();
    serial::print("GDT initialized.\n");
    
    idt::init();
    serial::print("IDT initialized.\n");
    
    pmm::init(128 * 1024 * 1024);
    serial::print("PMM initialized.\n");
    
    heap::init();
    serial::print("Heap initialized.\n");
    
    timer::init(100);
    serial::print("Timer initialized.\n");
    
    serial::print("VMM ready.\n");

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
