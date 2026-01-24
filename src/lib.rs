/***************************************************************************
 *
 * cJSON FFI BINDING FOR RUST
 * Copyright (C) 2026 Antonio Salsi <passy.linux@zresa.it>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 ***************************************************************************/

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

#[cfg(feature = "disable_panic")]
extern crate osal_rs;

#[cfg(feature = "disable_panic")]
extern crate osal_rs_serde;

pub(crate) mod cjson_ffi;
mod cjson;

pub(crate) mod cjson_utils_ffi;
mod cjson_utils;

#[cfg(feature = "osal_rs")]
pub mod ser;
#[cfg(feature = "osal_rs")]
pub mod de;

// Re-export main types for convenience
pub use cjson::{CJson, CJsonRef, CJsonResult, CJsonError};
pub use cjson_utils::{JsonPointer, JsonPatch, JsonMergePatch, JsonUtils};
#[cfg(feature = "osal_rs")]
use osal_rs_serde::{Result, Serialize};

#[cfg(feature = "osal_rs")]
use alloc::string::String;



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

#[cfg(feature = "osal_rs")]
pub fn to_json<T>(value: &T) -> Result<String> 
where 
    T: Serialize
{
    use crate::ser::JsonSerializer;
    use osal_rs::log_error;
    
    const APP_TAG: &str = "cJSON-RS";

    let mut serializer = JsonSerializer::new().map_err(|e| {
        log_error!(APP_TAG, "Failed to create JsonSerializer {}", e);
        osal_rs_serde::Error::InvalidData
    })?;

    value.serialize("", &mut serializer).map_err(|e| {
        log_error!(APP_TAG, "Serialization error: {}", e);
        osal_rs_serde::Error::InvalidData
    })?;
    
    let json = serializer.print_unformatted().map_err(|e| {
        log_error!(APP_TAG, "Failed to print JSON: {}", e);
        osal_rs_serde::Error::InvalidData
    })?;
    
    Ok(json)
}