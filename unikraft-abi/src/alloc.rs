use core::alloc::{GlobalAlloc, Layout};

#[derive(Debug)]
pub struct System;

unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc = *raw::_uk_alloc_head.get();
        let ptr = ((*alloc).malloc)(alloc, layout.size());
        assert_eq!(ptr.align_offset(layout.align()), 0);
        ptr.cast()
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let alloc = *raw::_uk_alloc_head.get();
        ((*alloc).free)(alloc, ptr.cast())
    }
}

#[cfg(feature = "global_allocator")]
#[global_allocator]
static GLOBAL: System = System;

/// From `unikraft/lib/ukalloc/include/alloc.h`
#[allow(non_camel_case_types)]
mod raw {
    use core::cell::UnsafeCell;
    use core::ffi::{c_int, c_void};

    pub type uk_alloc_malloc_func_t =
        unsafe extern "C" fn(a: *mut uk_alloc, size: usize) -> *mut c_void;

    pub type uk_alloc_calloc_func_t =
        unsafe extern "C" fn(a: *mut uk_alloc, nmemb: usize, size: usize) -> *mut c_void;

    pub type uk_alloc_posix_memalign_func_t = unsafe extern "C" fn(
        a: *mut uk_alloc,
        memptr: *mut *mut c_void,
        align: usize,
        size: usize,
    ) -> c_int;

    pub type uk_alloc_memalign_func_t =
        unsafe extern "C" fn(a: *mut uk_alloc, align: usize, size: usize) -> *mut c_void;

    pub type uk_alloc_realloc_func_t =
        unsafe extern "C" fn(a: *mut uk_alloc, ptr: *mut c_void, size: usize) -> *mut c_void;

    pub type uk_alloc_free_func_t = unsafe extern "C" fn(a: *mut uk_alloc, ptr: *mut c_void);

    #[repr(C)]
    pub struct uk_alloc {
        pub malloc: uk_alloc_malloc_func_t,
        pub calloc: uk_alloc_calloc_func_t,
        pub realloc: uk_alloc_realloc_func_t,
        pub posix_memalign: uk_alloc_posix_memalign_func_t,
        pub memalign: uk_alloc_memalign_func_t,
        pub free: uk_alloc_free_func_t,
    }

    extern "C" {
        pub static _uk_alloc_head: UnsafeCell<*mut uk_alloc>;
    }
}
