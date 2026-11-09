// Virtual Memory Manager - page table management for x86_64
// Implements 4-level paging with 4KB pages

const PAGE_SIZE: usize = 4096;

// Page table entry flags
pub const PAGE_PRESENT: u64 = 1 << 0;
pub const PAGE_WRITABLE: u64 = 1 << 1;
pub const PAGE_USER: u64 = 1 << 2;
pub const PAGE_WRITE_THROUGH: u64 = 1 << 3;
pub const PAGE_CACHE_DISABLE: u64 = 1 << 4;
pub const PAGE_ACCESSED: u64 = 1 << 5;
pub const PAGE_DIRTY: u64 = 1 << 6;
pub const PAGE_HUGE: u64 = 1 << 7;
pub const PAGE_GLOBAL: u64 = 1 << 8;

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [u64; 512],
}

impl PageTable {
    pub fn new() -> Self {
        Self { entries: [0; 512] }
    }

    pub fn get_entry(&self, index: usize) -> u64 {
        if index < 512 { self.entries[index] } else { 0 }
    }

    pub fn set_entry(&mut self, index: usize, value: u64) {
        if index < 512 { self.entries[index] = value; }
    }

    // Map a virtual address to a physical address with given flags
    // Creates intermediate page tables as needed (allocated from PMM)
    pub fn map(&mut self, virt: u64, phys: u64, flags: u64) {
        let pml4_idx = ((virt >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt >> 12) & 0x1FF) as usize;

        // PML4 -> PDPT
        let pdpt = if self.entries[pml4_idx] & PAGE_PRESENT != 0 {
            (self.entries[pml4_idx] & !0xFFF) as *mut PageTable
        } else {
            let addr = crate::pmm::alloc_page();
            if addr == 0 { return; }
            let pt = addr as *mut PageTable;
            unsafe { core::ptr::write_bytes(pt, 0, 1); }
            self.entries[pml4_idx] = addr as u64 | PAGE_PRESENT | PAGE_WRITABLE | PAGE_USER;
            pt
        };
        // PDPT -> PD
        unsafe {
            let pdpt_ref = &mut *pdpt;
            let pd = if pdpt_ref.entries[pdpt_idx] & PAGE_PRESENT != 0 {
                (pdpt_ref.entries[pdpt_idx] & !0xFFF) as *mut PageTable
            } else {
                let addr = crate::pmm::alloc_page();
                if addr == 0 { return; }
                let pt = addr as *mut PageTable;
                core::ptr::write_bytes(pt, 0, 1);
                pdpt_ref.entries[pdpt_idx] = addr as u64 | PAGE_PRESENT | PAGE_WRITABLE | PAGE_USER;
                pt
            };
            // PD -> PT
            let pd_ref = &mut *pd;
            let pt = if pd_ref.entries[pd_idx] & PAGE_PRESENT != 0 {
                (pd_ref.entries[pd_idx] & !0xFFF) as *mut PageTable
            } else {
                let addr = crate::pmm::alloc_page();
                if addr == 0 { return; }
                let pt = addr as *mut PageTable;
                core::ptr::write_bytes(pt, 0, 1);
                pd_ref.entries[pd_idx] = addr as u64 | PAGE_PRESENT | PAGE_WRITABLE | PAGE_USER;
                pt
            };
            // PT -> entry
            let pt_ref = &mut *pt;
            pt_ref.entries[pt_idx] = (phys & !0xFFF) | flags;
        }
    }

    // Unmap a virtual address
    pub fn unmap(&mut self, virt: u64) {
        let pml4_idx = ((virt >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt >> 12) & 0x1FF) as usize;

        unsafe {
            if self.entries[pml4_idx] & PAGE_PRESENT == 0 { return; }
            let pdpt = (self.entries[pml4_idx] & !0xFFF) as *mut PageTable;
            if (*pdpt).entries[pdpt_idx] & PAGE_PRESENT == 0 { return; }
            let pd = ((*pdpt).entries[pdpt_idx] & !0xFFF) as *mut PageTable;
            if (*pd).entries[pd_idx] & PAGE_PRESENT == 0 { return; }
            let pt = ((*pd).entries[pd_idx] & !0xFFF) as *mut PageTable;
            (*pt).entries[pt_idx] = 0;
        }
    }

    // Translate virtual address to physical
    pub fn translate(&self, virt: u64) -> Option<u64> {
        let pml4_idx = ((virt >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((virt >> 30) & 0x1FF) as usize;
        let pd_idx = ((virt >> 21) & 0x1FF) as usize;
        let pt_idx = ((virt >> 12) & 0x1FF) as usize;

        unsafe {
            if self.entries[pml4_idx] & PAGE_PRESENT == 0 { return None; }
            let pdpt = (self.entries[pml4_idx] & !0xFFF) as *const PageTable;
            if (*pdpt).entries[pdpt_idx] & PAGE_PRESENT == 0 { return None; }
            let pd = ((*pdpt).entries[pdpt_idx] & !0xFFF) as *const PageTable;
            if (*pd).entries[pd_idx] & PAGE_PRESENT == 0 { return None; }
            if (*pd).entries[pd_idx] & PAGE_HUGE != 0 {
                let phys = (*pd).entries[pd_idx] & !0x1FFFFF;
                return Some(phys | (virt & 0x1FFFFF));
            }
            let pt = ((*pd).entries[pd_idx] & !0xFFF) as *const PageTable;
            if (*pt).entries[pt_idx] & PAGE_PRESENT == 0 { return None; }
            let phys = (*pt).entries[pt_idx] & !0xFFF;
            Some(phys | (virt & 0xFFF))
        }
    }
}

// Switch to a new page table (load CR3)
pub fn switch_page_table(pml4_phys: u64) {
    unsafe {
        core::arch::asm!(
            "mov cr3, {}",
            in(reg) pml4_phys,
            options(nostack)
        );
    }
}

// Get current CR3
pub fn get_cr3() -> u64 {
    let cr3: u64;
    unsafe {
        core::arch::asm!("mov {}, cr3", out(reg) cr3, options(nostack));
    }
    cr3
}