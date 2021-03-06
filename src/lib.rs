#![no_std]

use core::alloc::{GlobalAlloc, Layout};

pub struct ZeroizingAllocator<Alloc: GlobalAlloc>(pub Alloc);

unsafe fn zero(ptr: *mut u8, size: usize) {
    for i in 0..size {
        core::ptr::write_volatile(ptr.offset(i as isize), 0);
    }
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

unsafe impl<T> GlobalAlloc for ZeroizingAllocator<T>
where
    T: GlobalAlloc,
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        zero(ptr, layout.size());
        #[cfg(not(test))]
        self.0.dealloc(ptr, layout);
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.0.alloc_zeroed(layout)
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::vec::Vec;

    #[global_allocator]
    static ALLOC: super::ZeroizingAllocator<std::alloc::System> =
        super::ZeroizingAllocator(std::alloc::System);

    #[test]
    fn test_static() {
        let mut a = Vec::with_capacity(2);
        a.push(0xde);
        a.push(0xad);
        let ptr1: *const u8 = &a[0];
        a.push(0xbe);
        a.push(0xef);
        let ptr2: *const u8 = &a[0];
        assert_eq!(&[0xde, 0xad, 0xbe, 0xef], &a[..]);
        assert_eq!(unsafe { ptr1.as_ref() }, Some(&0));
        drop(a);
        assert_eq!(unsafe { ptr2.as_ref() }, Some(&0));
    }

    quickcheck::quickcheck! {
        fn prop(v1: Vec<u8>, v2: Vec<u8>) -> bool {
            let mut v1 = v1;
            if v1.len() == 0 || v2.len() == 0 {
                return true;
            }
            let ptr1: *const u8 = &v1[0];
            v1.shrink_to_fit();
            let ptr2: *const u8 = &v2[0];
            v1.extend(v2);
            let ptr3: *const u8 = &v1[0];
            assert_eq!(unsafe { ptr1.as_ref() }, Some(&0));
            assert_eq!(unsafe { ptr2.as_ref() }, Some(&0));
            drop(v1);
            assert_eq!(unsafe { ptr3.as_ref() }, Some(&0));
            true
        }
    }
}
