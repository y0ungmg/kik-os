// IDT (Interrupt Descriptor Table) setup for x86_64

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

const IDT_ENTRIES: usize = 256;

static mut IDT: [IdtEntry; IDT_ENTRIES] = [IdtEntry {
    offset_low: 0,
    selector: 0,
    ist: 0,
    type_attr: 0,
    offset_mid: 0,
    offset_high: 0,
    reserved: 0,
}; IDT_ENTRIES];

// Exception handler stubs (simplified - just halt)
extern "C" fn exception_handler() {
    // In a real implementation, this would print diagnostics
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

pub fn init() {
    // Deferred: IDT setup causes issues under UEFI
    // Just return for now
}