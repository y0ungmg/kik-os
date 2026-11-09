// Physical Memory Manager (PMM) - simple bitmap allocator

const PAGE_SIZE: usize = 4096;
const MAX_PAGES: usize = 1024; // 4MB max (reduced for UEFI compatibility)

static mut BITMAP: [u64; MAX_PAGES / 64] = [0; MAX_PAGES / 64];
static mut TOTAL_PAGES: usize = 0;
static mut FREE_PAGES: usize = 0;

pub fn init(total_memory_bytes: usize) {
    unsafe {
        TOTAL_PAGES = total_memory_bytes / PAGE_SIZE;
        if TOTAL_PAGES > MAX_PAGES { TOTAL_PAGES = MAX_PAGES; }
        FREE_PAGES = TOTAL_PAGES;
        if TOTAL_PAGES > 0 {
            BITMAP[0] |= 1; // mark page 0 as used
        }
    }
}

fn set_page_used(page: usize) {
    unsafe {
        BITMAP[page / 64] |= 1 << (page % 64);
    }
}

fn set_page_free(page: usize) {
    unsafe {
        BITMAP[page / 64] &= !(1 << (page % 64));
    }
}

fn is_page_used(page: usize) -> bool {
    unsafe {
        (BITMAP[page / 64] >> (page % 64)) & 1 == 1
    }
}

pub fn is_page_used_pub(page: usize) -> bool {
    is_page_used(page)
}

pub fn set_page_used_pub(page: usize) {
    set_page_used(page)
}

pub fn alloc_page() -> usize {
    unsafe {
        let mut i = 0;
        while i < TOTAL_PAGES {
            if !is_page_used(i) {
                set_page_used(i);
                return i * PAGE_SIZE;
            }
            i += 1;
        }
        0 // 0 = failure (page 0 is always used)
    }
}

pub fn free_page(addr: usize) {
    let page = addr / PAGE_SIZE;
    unsafe {
        if is_page_used(page) {
            set_page_free(page);
            FREE_PAGES += 1;
        }
    }
}

pub fn total_pages_pub() -> usize {
    unsafe { TOTAL_PAGES }
}

pub fn free_pages_count() -> usize {
    unsafe { FREE_PAGES }
}
