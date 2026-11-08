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
    
    keyboard::init();
    serial::print("Keyboard initialized.\n");
    
    pci::enumerate();
    serial::print("PCI enumerated.\n");
    
    kikfs::init();
    serial::print("KiKFS initialized.\n");
    
    net::init();
    serial::print("Network initialized.\n");
    
    audio::init();
    serial::print("Audio initialized.\n");
    
    usb::init();
    serial::print("USB initialized.\n");
    
    power::init();
    serial::print("Power management initialized.\n");
    
    users::init();
    serial::print("User management initialized.\n");
    
    scheduler::init();
    serial::print("Scheduler initialized.\n");
    
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