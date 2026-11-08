// Kernel Heap Allocator - simple bump allocator

use core::alloc::{GlobalAlloc, Layout};

const HEAP_SIZE: usize = 1024 * 1024; // 1MB heap
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
static mut HEAP_POS: usize = 0;

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        unsafe {
            let pos = (HEAP_POS + align - 1) & !(align - 1);
            if pos + size <= HEAP_SIZE {
                HEAP_POS = pos + size;
                (&raw const HEAP as *const u8).add(pos) as *mut u8
            } else {
                core::ptr::null_mut()
            }
        }
    }
    
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't free individual allocations
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

pub fn init() {
    // Heap is already initialized statically
}