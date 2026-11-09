// Task State Segment (TSS) for x86_64
#[repr(C, packed)]
pub struct Tss {
    pub reserved0: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    pub reserved1: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub reserved2: u64,
    pub reserved3: u16,
    pub iopb_offset: u16,
    // I/O permission bitmap (8192 bytes = 65536 ports / 8)
    // All zeros = all ports accessible from ring 3
    pub iopb: [u8; 8192],
}

static mut TSS: Tss = Tss {
    reserved0: 0,
    rsp0: 0,
    rsp1: 0,
    rsp2: 0,
    reserved1: 0,
    ist1: 0,
    ist2: 0,
    ist3: 0,
    ist4: 0,
    ist5: 0,
    ist6: 0,
    ist7: 0,
    reserved2: 0,
    reserved3: 0,
    iopb_offset: 104, // Offset of iopb field within TSS (0x68)
    iopb: [0; 8192],
};

pub fn init(kernel_stack: u64) {
    unsafe {
        TSS.rsp0 = kernel_stack;
        // iopb_offset already set to 104 (0x68)
    }
}

pub fn get_tss_addr() -> u64 {
    &raw const TSS as *const _ as u64
}

pub fn get_tss_size() -> usize {
    core::mem::size_of::<Tss>()
}