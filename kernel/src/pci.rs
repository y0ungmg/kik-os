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
    for bus in 0..=255u8 {
        for slot in 0..32u8 {
            let vendor_id = read_config_word(bus, slot, 0, 0);
            if vendor_id != 0xFFFF {
                let device_id = read_config_word(bus, slot, 0, 2);
                let class_code = read_config_dword(bus, slot, 0, 8) >> 24;
                let _ = (device_id, class_code);
                // In a real implementation, we'd store these
            }
        }
    }
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