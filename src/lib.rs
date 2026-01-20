#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

#[cfg(feature = "disable_panic")]
extern crate osal_rs;

pub(crate) mod cjson_ffi;
mod cjson;

pub(crate) mod cjson_utils_ffi;
mod cjson_utils;

// Re-export main types for convenience
pub use cjson::{CJson, CJsonRef, CJsonResult, CJsonError};
pub use cjson_utils::{JsonPointer, JsonPatch, JsonMergePatch, JsonUtils};

// Global allocator for no_std environments (disabled when disable_panic is enabled)
#[cfg(all(not(any(test, feature = "std")), not(feature = "disable_panic")))]
use core::alloc::Layout;

#[cfg(all(not(any(test, feature = "std")), not(feature = "disable_panic")))]
struct CJsonAllocator;

#[cfg(all(not(any(test, feature = "std")), not(feature = "disable_panic")))]
unsafe impl core::alloc::GlobalAlloc for CJsonAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe extern "C" {
            fn malloc(size: usize) -> *mut u8;
        }
        unsafe { malloc(layout.size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe extern "C" {
            fn free(ptr: *mut u8);
        }
        unsafe { free(ptr) }
    }
}

#[cfg(all(not(any(test, feature = "std")), not(feature = "disable_panic")))]
#[global_allocator]
static ALLOCATOR: CJsonAllocator = CJsonAllocator;

// Panic handler for no_std environments (can be disabled with disable_panic feature)
#[cfg(all(not(any(test, feature = "std")), not(feature = "disable_panic")))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

