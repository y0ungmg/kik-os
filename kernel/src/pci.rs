// PCI/PCIe enumeration

const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

pub fn read_config_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let address = 0x80000000u32
        | ((bus as u32) << 16)
        | ((slot as u32) << 11)
        | ((func as u32) << 8)
        | (offset as u32 & 0xFC);
    
    unsafe {
        outl(PCI_CONFIG_ADDRESS, address);
        inw(PCI_CONFIG_DATA + (offset as u16 & 2))
    }
}

pub fn read_config_dword(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address = 0x80000000u32
        | ((bus as u32) << 16)
        | ((slot as u32) << 11)
        | ((func as u32) << 8)
        | (offset as u32 & 0xFC);
    
    unsafe {
        outl(PCI_CONFIG_ADDRESS, address);
        inl(PCI_CONFIG_DATA)
    }
}

pub fn enumerate() {
    crate::serial::print("PCI: starting enumeration...\n");
    // Only check slot 0, 1, ... with a simple loop
    let mut slot = 0u8;
    while slot < 32 {
        let vendor_id = read_config_word(0, slot, 0, 0);
        if vendor_id != 0xFFFF {
            crate::serial::print("PCI: found device at slot ");
            let hi = (slot >> 4) & 0x0F;
            let lo = slot & 0x0F;
            let hex_chars = b"0123456789ABCDEF";
            crate::serial::write_byte(hex_chars[hi as usize]);
            crate::serial::write_byte(hex_chars[lo as usize]);
            crate::serial::print("\n");
        }
        slot += 1;
    }
    crate::serial::print("PCI: enumeration complete.\n");
}

unsafe fn outl(port: u16, value: u32) {
    unsafe {
        core::arch::asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

unsafe fn inl(port: u16) -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!(
            "in eax, dx",
            out("eax") value,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

unsafe fn inw(port: u16) -> u16 {
    let value: u16;
    unsafe {
        core::arch::asm!(
            "in ax, dx",
            out("ax") value,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}