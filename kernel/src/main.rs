// Kernel binary entry point
#![no_std]
#![no_main]

mod serial;
mod log;
mod gdt;
mod idt;
mod pmm;
mod heap;
mod timer;
mod vmm;
mod scheduler;
mod syscalls;
mod keyboard;
mod pci;
mod framebuffer;
mod kikfs;
mod net;
mod audio;
mod usb;
mod power;
mod ipc;
mod users;

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
    
    pmm::init(1024 * 1024); // 1MB (reduced for UEFI static-write compat)
    serial::print("PMM initialized.\n");
    
    heap::init();
    serial::print("Heap initialized.\n");
    
    timer::init(100);
    serial::print("Timer initialized.\n");
    
    keyboard::init();
    serial::print("Keyboard initialized.\n");
    
    // pci::enumerate(); // deferred - takes too long under QEMU
    serial::print("PCI skipped.\n");
    
    kikfs::init();
    serial::print("KiKFS initialized.\n");
    
    net::init();
    serial::print("Network initialized.\n");
    
    audio::init();
    serial::print("Audio initialized.\n");
    
    usb::init();
    serial::print("USB initialized.\n");
    
    users::init();
    serial::print("Users initialized.\n");
    
    power::init();
    serial::print("Power initialized.\n");
    
    scheduler::init();
    serial::print("Scheduler initialized.\n");
    
    // Test alloc_page
    serial::print("Testing alloc_page...\n");
    let tp = pmm::total_pages_pub();
    serial::print("total_pages checked.\n");
    let page = pmm::alloc_page();
    serial::print("alloc_page returned.\n");
    if page != 0 {
        serial::print("PMM alloc_page test passed.\n");
        pmm::free_page(page);
    } else {
        serial::print("PMM alloc_page returned 0!\n");
    }
    
    // Test IPC
    let msg = ipc::Message {
        sender: 1,
        receiver: 2,
        msg_type: 0,
        size: 0,
        data: [0; 64],
    };
    if ipc::send_message(msg) {
        serial::print("IPC send test passed.\n");
    }
    
    // Test syscalls
    let r1 = syscalls::handle_syscall(1, 1, 0, 0);
    let r2 = syscalls::handle_syscall(39, 0, 0, 0);
    if r1 == 0 && r2 == 1 {
        serial::print("Syscall tests passed.\n");
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
