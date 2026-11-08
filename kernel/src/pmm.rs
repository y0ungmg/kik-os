// Physical Memory Manager (PMM) - simple bitmap allocator

// Removed unused imports

const PAGE_SIZE: usize = 4096;
const MAX_PAGES: usize = 1024 * 1024; // 4GB max

static mut BITMAP: [u64; MAX_PAGES / 64] = [0; MAX_PAGES / 64];
static mut TOTAL_PAGES: usize = 0;
static mut FREE_PAGES: usize = 0;

pub fn init(total_memory_bytes: usize) {
    unsafe {
        TOTAL_PAGES = total_memory_bytes / PAGE_SIZE;
        FREE_PAGES = TOTAL_PAGES;
        // Mark all pages as free (0 = free in our bitmap, 1 = used)
        for i in 0..(TOTAL_PAGES / 64 + 1) {
            BITMAP[i] = 0;
        }
        // Mark first 1MB as used (kernel, bootloader, etc.)
        let reserved_pages = 0x100000 / PAGE_SIZE;
        for i in 0..reserved_pages {
            set_page_used(i);
            FREE_PAGES -= 1;
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

pub fn alloc_page() -> Option<usize> {
    unsafe {
        for i in 0..TOTAL_PAGES {
            if !is_page_used(i) {
                set_page_used(i);
                FREE_PAGES -= 1;
                return Some(i * PAGE_SIZE);
            }
        }
        None
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

pub fn free_pages_count() -> usize {
    unsafe { FREE_PAGES }
}

pub fn total_pages_count() -> usize {
    unsafe { TOTAL_PAGES }
}