// Programmable Interrupt Controller (PIC) management

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

pub fn init() {
    unsafe {
        // Save masks
        let _a1 = inb(PIC1_DATA);
        let _a2 = inb(PIC2_DATA);
        
        // Start initialization sequence
        outb(PIC1_COMMAND, 0x11);
        outb(PIC2_COMMAND, 0x11);
        
        // Set vector offsets (IRQ0 -> 32, IRQ8 -> 40)
        outb(PIC1_DATA, 32);
        outb(PIC2_DATA, 40);
        
        // Configure cascading
        outb(PIC1_DATA, 4);
        outb(PIC2_DATA, 2);
        
        // Set 8086 mode
        outb(PIC1_DATA, 0x01);
        outb(PIC2_DATA, 0x01);
        
        // Unmask timer IRQ (IRQ0)
        unsafe {
            outb(PIC1_DATA, 0xFF); // Mask all IRQs
            outb(PIC2_DATA, 0xFF);
        }
    }
}
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, 0x20);
        }
        outb(PIC1_COMMAND, 0x20);
    }
}

fn io_wait() {
    unsafe {
        outb(0x80, 0);
    }
}

unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack));
}

unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nomem, nostack));
    val
}