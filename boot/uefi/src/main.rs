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

// ELF64 header
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

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    
    uefi::system::with_stdout(|stdout| {
        let _ = stdout.write_str("KiK Boot - UEFI\n");
        let _ = stdout.write_str("Loading KiK OS kernel...\n");
    });
    
    // Parse ELF header
    let elf_header = unsafe { &*(KERNEL.as_ptr() as *const Elf64Header) };
    
    // Verify ELF magic
    if elf_header.magic != [0x7F, b'E', b'L', b'F'] {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.write_str("Invalid ELF magic!\n");
        });
        loop {}
    }
    
    uefi::system::with_stdout(|stdout| {
        let _ = stdout.write_str("ELF header valid.\n");
        let _ = stdout.write_str("Entry point found.\n");
    });
    
    // In a real implementation, we'd:
    // 1. Iterate through program headers
    // 2. Load PT_LOAD segments into memory using boot services
    // 3. Jump to entry point
    // For now, just hang after validation
    loop {}
}