// GDT (Global Descriptor Table) setup for x86_64

use core::mem::size_of;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GdtEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_mid: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
}

#[repr(C, packed)]
pub struct GdtPointer {
    pub limit: u16,
    pub base: u64,
}

// GDT entries
static mut GDT: [GdtEntry; 5] = [
    // Null descriptor
    GdtEntry { limit_low: 0, base_low: 0, base_mid: 0, access: 0, granularity: 0, base_high: 0 },
    // Kernel code segment (64-bit)
    GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x9A, granularity: 0xAF, base_high: 0 },
    // Kernel data segment
    GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0x92, granularity: 0xCF, base_high: 0 },
    // User code segment (64-bit)
    GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0xFA, granularity: 0xAF, base_high: 0 },
    // User data segment
    GdtEntry { limit_low: 0xFFFF, base_low: 0, base_mid: 0, access: 0xF2, granularity: 0xCF, base_high: 0 },
];

pub fn init() {
    let gdt_ptr = GdtPointer {
        limit: (size_of::<[GdtEntry; 5]>() - 1) as u16,
        base: unsafe { &raw const GDT as *const _ as u64 },
    };
    
    unsafe {
        core::arch::asm!(
            "lgdt [{}]",
            in(reg) &gdt_ptr,
            options(nostack)
        );
        
        // Reload segment registers
        core::arch::asm!(
            "mov ax, 0x10",  // Kernel data segment
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            options(nostack)
        );
        
        // Far jump to reload CS
        core::arch::asm!(
            "push 0x08",     // Kernel code segment
            "lea rax, [rip + 2f]",
            "push rax",
            "retfq",
            "2:",
            options(nostack)
        );
    }
}