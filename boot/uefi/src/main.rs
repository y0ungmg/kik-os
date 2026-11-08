#![no_std]
#![no_main]

use uefi::prelude::*;
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static KERNEL: &[u8] = include_bytes!("../../../target/x86_64-unknown-none/debug/kik-kernel");

#[repr(C)]
struct Elf64Header {
    magic: [u8; 4],
    class: u8,
    data: u8,
    version: u8,
    osabi: u8,
    abiversion: u8,
    pad: [u8; 7],
    etype: u16,
    machine: u16,
    e_version: u32,
    entry: u64,
    phoff: u64,
    shoff: u64,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

#[repr(C)]
struct Elf64ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

const PT_LOAD: u32 = 1;

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

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.write_str("KiK Boot - UEFI\n");
        let _ = stdout.write_str("Loading KiK OS kernel...\n");
    });

    let elf_header = unsafe { &*(KERNEL.as_ptr() as *const Elf64Header) };

    if elf_header.magic != [0x7F, b'E', b'L', b'F'] {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.write_str("Invalid ELF magic!\n");
        });
        loop {}
    }

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.write_str("ELF header valid.\n");
    });

    // Calculate total kernel size from ELF segments
    let mut max_addr: u64 = 0x100000;
    let phdr_base = unsafe { KERNEL.as_ptr().add(elf_header.phoff as usize) };
    for i in 0..elf_header.phnum {
        let phdr = unsafe {
            &*(phdr_base.add(i as usize * elf_header.phentsize as usize) as *const Elf64ProgramHeader)
        };
        if phdr.p_type == PT_LOAD {
            let end = phdr.p_vaddr + phdr.p_memsz;
            if end > max_addr {
                max_addr = end;
            }
        }
    }
    let kernel_pages = ((max_addr - 0x100000) as usize + 0xFFF) / 0x1000 + 1;

    // Allocate memory at 0x100000 for the kernel
    let alloc_result = uefi::boot::allocate_pages(
        uefi::boot::AllocateType::Address(0x100000),
        uefi::boot::MemoryType::LOADER_DATA,
        kernel_pages,
    );
    uefi::system::with_stdout(|stdout| {
        if alloc_result.is_ok() {
            let _ = stdout.write_str("Memory allocated at 0x100000.\n");
        } else {
            let _ = stdout.write_str("Memory allocation failed!\n");
        }
    });

    // Load PT_LOAD segments
    let phdr_base = unsafe { KERNEL.as_ptr().add(elf_header.phoff as usize) };
    for i in 0..elf_header.phnum {
        let phdr = unsafe {
            &*(phdr_base.add(i as usize * elf_header.phentsize as usize) as *const Elf64ProgramHeader)
        };
        if phdr.p_type == PT_LOAD {
            let src = unsafe { KERNEL.as_ptr().add(phdr.p_offset as usize) };
            let dst = phdr.p_vaddr as *mut u8;
            unsafe {
                core::ptr::copy_nonoverlapping(src, dst, phdr.p_filesz as usize);
                if phdr.p_memsz > phdr.p_filesz {
                    core::ptr::write_bytes(dst.add(phdr.p_filesz as usize), 0, (phdr.p_memsz - phdr.p_filesz) as usize);
                }
            }
        }
    }

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.write_str("Kernel loaded at 0x100000.\n");
    });

    // Create BootInfo
    let boot_info = BootInfo {
        magic: 0x4B694B4F53, // "KiKOS"
        kernel_load_addr: 0x100000,
        kernel_size: KERNEL.len() as u64,
        memory_map_addr: 0,
        memory_map_count: 0,
        framebuffer_addr: 0,
        framebuffer_width: 0,
        framebuffer_height: 0,
        framebuffer_pitch: 0,
        framebuffer_bpp: 0,
        boot_mode: 1, // UEFI
    };

    let boot_info_ptr = &boot_info as *const BootInfo;

    uefi::system::with_stdout(|stdout| {
        let _ = write!(stdout, "Entry point: {:x}\n", elf_header.entry);
        let _ = stdout.write_str("Jumping to kernel now...\n");
    });

    // Jump to kernel entry point
    let entry: extern "C" fn(*const BootInfo) -> ! = unsafe {
        core::mem::transmute(elf_header.entry)
    };

    entry(boot_info_ptr);

    // Should never reach here
    loop {}
}