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
mod tss;
mod usermode;
mod pic;
mod process;
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
    
    pic::init();
    serial::print("PIC initialized.\n");
    
    pmm::init(1024 * 1024); // 1MB (reduced for UEFI static-write compat)
    serial::print("PMM initialized.\n");
    
    heap::init();
    serial::print("Heap initialized.\n");
    
    serial::print("Before timer init.\n");
    timer::init(100);
    serial::print("Timer initialized.\n");
    
    // Enable interrupts
    unsafe {
        core::arch::asm!("sti");
    }
    serial::print("Interrupts enabled.\n");
    
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
    
    tss::init(0x200000); // kernel stack at 2MB
    serial::print("TSS initialized.\n");
    
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
    
    // Test framebuffer from boot info
    if !_boot_info.is_null() {
        let bi = unsafe { &*_boot_info };
        if bi.framebuffer_addr != 0 {
            let fb = framebuffer::Framebuffer::new(
                bi.framebuffer_addr,
                bi.framebuffer_width,
                bi.framebuffer_height,
                bi.framebuffer_pitch,
                bi.framebuffer_bpp,
            );
            // Draw a red pixel at (10, 10)
            fb.set_pixel(10, 10, 0x00FF0000);
            // Draw a blue rectangle
            let mut y = 50;
            while y < 100 {
                let mut x = 50;
                while x < 150 {
                    fb.set_pixel(x, y, 0x000000FF);
                    x += 1;
                }
                y += 1;
            }
            serial::print("Framebuffer test: pixel drawn.\n");
        }
    }
    
    // Test process management
    let pid = process::create_process(0x1000);
    if pid != 0 {
        serial::print("Process creation test passed.\n");
    }
    
    serial::print("Hello from KiK OS!\n");
    
    // Main loop - check for keyboard input
    let mut last = 0u8;
    loop {
        let sc = idt::get_last_scancode();
        if sc != 0 && sc != last {
            last = sc;
            // Print scancode as hex
            serial::print("Key: ");
            // Simple hex print
            let hi = (sc >> 4) & 0x0F;
            let lo = sc & 0x0F;
            let hex_chars = b"0123456789ABCDEF";
            serial::write_byte(hex_chars[hi as usize]);
            serial::write_byte(hex_chars[lo as usize]);
            serial::print("\n");
        }
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
