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

const IDT_ENTRIES: usize = 48; // 32 exceptions + 16 IRQs

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

extern "C" fn irq_handler() {
    // Timer IRQ (IRQ0)
    // Send EOI
    unsafe {
        // EOI to PIC1
        core::arch::asm!("out 0x20, al", in("al") 0x20u8, options(nomem, nostack));
    }
}

pub fn init() {
    let mut i = 0;
    while i < IDT_ENTRIES {
        let handler_addr = if i < 32 {
            exception_handler as u64
        } else {
            irq_handler as u64
        };
        unsafe {
            IDT[i].offset_low = (handler_addr & 0xFFFF) as u16;
            IDT[i].selector = 0x08;
            IDT[i].ist = 0;
            IDT[i].type_attr = 0x8E;
            IDT[i].offset_mid = ((handler_addr >> 16) & 0xFFFF) as u16;
            IDT[i].offset_high = (handler_addr >> 32) as u32;
            IDT[i].reserved = 0;
        }
        i += 1;
    }
    let idt_ptr = IdtPointer {
        limit: (size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16,
        base: &raw const IDT as *const _ as u64,
    };
    
    unsafe {
        core::arch::asm!(
            "lidt [{}]",
            in(reg) &idt_ptr,
            options(nostack)
        );
    }
    
    let _ = (IDT_ENTRIES, exception_handler as u64, size_of::<IdtEntry>());
}
