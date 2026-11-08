// IDT (Interrupt Descriptor Table) setup for x86_64
// Deferred: static array writes hang under UEFI

use core::mem::size_of;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub ist: u8,
    pub type_attr: u8,
    pub offset_mid: u16,
    pub offset_high: u32,
    pub reserved: u32,
}

#[repr(C, packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64,
}

const IDT_ENTRIES: usize = 32;

static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry {
    offset_low: 0,
    selector: 0,
    ist: 0,
    type_attr: 0,
    offset_mid: 0,
    offset_high: 0,
    reserved: 0,
}; IDT_ENTRIES];

extern "C" fn exception_handler() {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

pub fn init() {
    // Deferred under UEFI: writing to static arrays in .bss hangs
    // even with identity-mapped page tables. The memory may not be
    // backed by RAM. Will be revisited with bootloader-allocated memory.
    let _ = (IDT_ENTRIES, exception_handler as u64, size_of::<IdtEntry>());
}
