use std::alloc::{GlobalAlloc, Layout, System};

struct ZeroizingAllocator;

unsafe impl GlobalAlloc for ZeroizingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        for i in 0..layout.size() {
            core::ptr::write_volatile(ptr.offset(i as isize), 0);
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: ZeroizingAllocator = ZeroizingAllocator;
