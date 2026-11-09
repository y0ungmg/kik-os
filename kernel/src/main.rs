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
mod ata;
mod usb;
mod power;
mod ipc;
mod tss;
mod usermode;
mod pic;
mod process;
mod window;
mod compositor;
mod users;

use core::panic::PanicInfo;

#[repr(C)]
#[derive(Copy, Clone)]
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
    
    gdt::init(tss::get_tss_addr(), (core::mem::size_of::<tss::Tss>() - 1) as u32);
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
    
    ata::init();
    serial::print("ATA storage initialized.\n");
    
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
    
    // Draw window on framebuffer (info from bootloader at 0x9000)
    let bi_ptr = 0x9000 as *const BootInfo;
    let fb_addr = unsafe { (*bi_ptr).framebuffer_addr };
    let fb_pitch = unsafe { (*bi_ptr).framebuffer_pitch };
    // Create compositor with framebuffer
    let fb = framebuffer::Framebuffer::new(fb_addr, 1280, 800, fb_pitch, 32);
    let mut comp = compositor::Compositor::new(fb);
    // Add two windows
    comp.add_window(window::Window::new(50, 50, 100, 80, "Window 1"));
    comp.add_window(window::Window::new(200, 80, 120, 90, "Window 2"));
    comp.draw_all();
    serial::print("Compositor: 2 windows drawn.\n");
    
    // Test VMM: read CR3
    let cr3 = vmm::get_cr3();
    serial::print("VMM: CR3 read OK.\n");
    serial::print("VMM test done.\n");
    
    // Test process management
    let pid = process::create_process(0x1000);
    if pid != 0 {
        serial::print("Process creation test passed.\n");
    }
    
    serial::print("Hello from KiK OS!\n");
    serial::print("> ");
    
    // User mode tested successfully (prints 'U' to serial) - see user_main()
    // Uncomment to test: usermode::switch_to_user_mode(user_main as u64, 0x300000);
    
    // Main loop - check for keyboard input
    let mut input_line = [0u8; 128];
    let mut input_pos = 0usize;
    loop {
        if keyboard::has_input() {
            if let Some(c) = keyboard::get_char() {
                if c == 13 { // Enter
                    serial::print("\n");
                    // Process command
                    let mut cmd = [0u8; 128];
                    let mut ci = 0;
                    while ci < input_pos {
                        cmd[ci] = input_line[ci];
                        ci += 1;
                    }
                    cmd[ci] = 0; // null terminate
                    // Simple command processing
                    if input_pos == 0 {
                        // Empty line
                    } else if input_line[0] == b'h' && input_line[1] == b'e' && input_line[2] == b'l' && input_line[3] == b'p' {
                        serial::print("KiK OS Shell Commands:\n");
                        serial::print("  help   - show this help\n");
                        serial::print("  clear  - clear screen\n");
                        serial::print("  ver    - show version\n");
                        serial::print("  echo   - echo text\n");
                    } else if input_line[0] == b'v' && input_line[1] == b'e' && input_line[2] == b'r' {
                        serial::print("KiK OS v0.1.0 - Bare metal Rust kernel\n");
                    } else if input_line[0] == b'c' && input_line[1] == b'l' && input_line[2] == b'e' && input_line[3] == b'a' && input_line[4] == b'r' {
                        // Clear screen (ANSI escape)
                        serial::print("\x1B[2J\x1B[H");
                    } else {
                        serial::print("Unknown command. Type 'help' for commands.\n");
                    }
                    serial::print("> ");
                    input_pos = 0;
                } else if c == 8 { // Backspace
                    if input_pos > 0 {
                        input_pos -= 1;
                        serial::write_byte(8);
                        serial::write_byte(b' ');
                        serial::write_byte(8);
                    }
                } else {
                    if input_pos < 127 {
                        input_line[input_pos] = c;
                        input_pos += 1;
                        serial::write_byte(c);
                    }
                }
            }
        }
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn user_main() -> ! {
    // Write 'U' to serial port (I/O allowed via TSS IOPB)
    unsafe {
        core::arch::asm!(
            "mov dx, 0x3F8",
            "mov al, 0x55",  // 'U'
            "out dx, al",
        );
    }
    loop {
        unsafe {
            core::arch::asm!("nop");
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
