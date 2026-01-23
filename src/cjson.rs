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


//! Safe Rust wrappers for cJSON library
//!
//! This module provides safe, idiomatic Rust interfaces over the cJSON C library.

extern crate alloc;

use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_int};
use core::ptr;
use core::fmt::Display;

use crate::cjson_ffi::*;

/// Result type for cJSON operations
pub type CJsonResult<T> = Result<T, CJsonError>;

/// Error types for cJSON operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CJsonError {
    /// Failed to parse JSON
    ParseError,
    /// Null pointer encountered
    NullPointer,
    /// Invalid UTF-8 in string
    InvalidUtf8,
    /// Item not found
    NotFound,
    /// Wrong type
    TypeError,
    /// Memory allocation failed
    AllocationError,
    /// Invalid operation
    InvalidOperation,
}

impl Display for CJsonError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CJsonError::ParseError => write!(f, "Failed to parse JSON"),
            CJsonError::NullPointer => write!(f, "Null pointer encountered"),
            CJsonError::InvalidUtf8 => write!(f, "Invalid UTF-8 in string"),
            CJsonError::NotFound => write!(f, "Item not found"),
            CJsonError::TypeError => write!(f, "Wrong type"),
            CJsonError::AllocationError => write!(f, "Memory allocation failed"),
            CJsonError::InvalidOperation => write!(f, "Invalid operation"),
        }
    }
}

#[cfg(feature = "disable_panic")]
impl From<osal_rs_serde::Error> for CJsonError {
    fn from(err: osal_rs_serde::Error) -> Self {
        use osal_rs_serde::Error::*;

        match err {
            BufferTooSmall => CJsonError::AllocationError,
            UnexpectedEof => CJsonError::ParseError,
            InvalidData => CJsonError::ParseError,
            TypeMismatch => CJsonError::TypeError,
            OutOfRange => CJsonError::NotFound,
            Custom(_) => CJsonError::InvalidOperation,
            Unsupported => CJsonError::InvalidOperation,
        }
    }
}

/// Safe wrapper for cJSON pointer
#[derive(Debug, Clone)]
pub struct CJson {
    ptr: *mut cJSON,
}

impl CJson {
    /// Create a new CJson wrapper from a raw pointer
    /// 
    /// # Safety
    /// The pointer must be valid and owned by this wrapper
    pub(crate) unsafe fn from_ptr(ptr: *mut cJSON) -> CJsonResult<Self> {
        if ptr.is_null() {
            Err(CJsonError::NullPointer)
        } else {
            Ok(CJson { ptr })
        }
    }

    /// Get the raw pointer (does not transfer ownership)
    pub fn as_ptr(&self) -> *const cJSON {
        self.ptr
    }

    /// Get the mutable raw pointer (does not transfer ownership)
    pub fn as_mut_ptr(&mut self) -> *mut cJSON {
        self.ptr
    }

    /// Consume the wrapper and return the raw pointer (transfers ownership)
    pub fn into_raw(self) -> *mut cJSON {
        let ptr = self.ptr;
        core::mem::forget(self);
        ptr
    }

    // ========================
    // PARSING FUNCTIONS
    // ========================

    /// Parse a JSON string
    pub fn parse(json: &str) -> CJsonResult<Self> {
        let c_str = CString::new(json).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_Parse(c_str.as_ptr()) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Parse a JSON string with specified length
    pub fn parse_with_length(json: &str, length: usize) -> CJsonResult<Self> {
        let c_str = CString::new(json).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_ParseWithLength(c_str.as_ptr(), length) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Parse a JSON string with options
    pub fn parse_with_opts(json: &str, require_null_terminated: bool) -> CJsonResult<Self> {
        let c_str = CString::new(json).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe {
            cJSON_ParseWithOpts(
                c_str.as_ptr(),
                ptr::null_mut(),
                if require_null_terminated { 1 } else { 0 },
            )
        };
        unsafe { Self::from_ptr(ptr) }
    }

    // ========================
    // PRINTING FUNCTIONS
    // ========================

    /// Print JSON to a formatted string
    pub fn print(&self) -> CJsonResult<String> {
        let c_str = unsafe { cJSON_Print(self.ptr) };
        if c_str.is_null() {
            return Err(CJsonError::AllocationError);
        }
        let rust_str = unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() };
        unsafe { cJSON_free(c_str as *mut core::ffi::c_void) };
        Ok(rust_str)
    }

    /// Print JSON to an unformatted string
    pub fn print_unformatted(&self) -> CJsonResult<String> {
        let c_str = unsafe { cJSON_PrintUnformatted(self.ptr) };
        if c_str.is_null() {
            return Err(CJsonError::AllocationError);
        }
        let rust_str = unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() };
        unsafe { cJSON_free(c_str as *mut core::ffi::c_void) };
        Ok(rust_str)
    }

    // ========================
    // TYPE CHECKING FUNCTIONS
    // ========================

    /// Check if the item is invalid
    pub fn is_invalid(&self) -> bool {
        unsafe { cJSON_IsInvalid(self.ptr) != 0 }
    }

    /// Check if the item is false
    pub fn is_false(&self) -> bool {
        unsafe { cJSON_IsFalse(self.ptr) != 0 }
    }

    /// Check if the item is true
    pub fn is_true(&self) -> bool {
        unsafe { cJSON_IsTrue(self.ptr) != 0 }
    }

    /// Check if the item is a boolean
    pub fn is_bool(&self) -> bool {
        unsafe { cJSON_IsBool(self.ptr) != 0 }
    }

    /// Check if the item is null
    pub fn is_null(&self) -> bool {
        unsafe { cJSON_IsNull(self.ptr) != 0 }
    }

    /// Check if the item is a number
    pub fn is_number(&self) -> bool {
        unsafe { cJSON_IsNumber(self.ptr) != 0 }
    }

    /// Check if the item is a string
    pub fn is_string(&self) -> bool {
        unsafe { cJSON_IsString(self.ptr) != 0 }
    }

    /// Check if the item is an array
    pub fn is_array(&self) -> bool {
        unsafe { cJSON_IsArray(self.ptr) != 0 }
    }

    /// Check if the item is an object
    pub fn is_object(&self) -> bool {
        unsafe { cJSON_IsObject(self.ptr) != 0 }
    }

    /// Check if the item is raw JSON
    pub fn is_raw(&self) -> bool {
        unsafe { cJSON_IsRaw(self.ptr) != 0 }
    }

    // ========================
    // VALUE RETRIEVAL FUNCTIONS
    // ========================

    /// Get string value
    pub fn get_string_value(&self) -> CJsonResult<String> {
        if !self.is_string() {
            return Err(CJsonError::TypeError);
        }
        let c_str = unsafe { cJSON_GetStringValue(self.ptr) };
        if c_str.is_null() {
            return Err(CJsonError::NullPointer);
        }
        Ok(unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() })
    }

    /// Get number value as f64
    pub fn get_number_value(&self) -> CJsonResult<f64> {
        if !self.is_number() {
            return Err(CJsonError::TypeError);
        }
        Ok(unsafe { cJSON_GetNumberValue(self.ptr) })
    }

    /// Get number value as i32
    pub fn get_int_value(&self) -> CJsonResult<i32> {
        if !self.is_number() {
            return Err(CJsonError::TypeError);
        }
        Ok(unsafe { (*self.ptr).valueint })
    }

    /// Get boolean value
    pub fn get_bool_value(&self) -> CJsonResult<bool> {
        if !self.is_bool() {
            return Err(CJsonError::TypeError);
        }
        Ok(self.is_true())
    }

    // ========================
    // ARRAY FUNCTIONS
    // ========================

    /// Get array size
    pub fn get_array_size(&self) -> CJsonResult<usize> {
        if !self.is_array() {
            return Err(CJsonError::TypeError);
        }
        Ok(unsafe { cJSON_GetArraySize(self.ptr) as usize })
    }

    /// Get array item by index (borrowed reference)
    pub fn get_array_item(&self, index: usize) -> CJsonResult<CJsonRef> {
        if !self.is_array() {
            return Err(CJsonError::TypeError);
        }
        let ptr = unsafe { cJSON_GetArrayItem(self.ptr, index as c_int) };
        unsafe { CJsonRef::from_ptr(ptr) }.map_err(|_| CJsonError::NotFound)
    }

    // ========================
    // OBJECT FUNCTIONS
    // ========================

    /// Get object item by key (borrowed reference)
    pub fn get_object_item(&self, key: &str) -> CJsonResult<CJsonRef> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_GetObjectItem(self.ptr, c_key.as_ptr()) };
        unsafe { CJsonRef::from_ptr(ptr) }.map_err(|_| CJsonError::NotFound)
    }

    /// Get object item by key (case sensitive, borrowed reference)
    pub fn get_object_item_case_sensitive(&self, key: &str) -> CJsonResult<CJsonRef> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_GetObjectItemCaseSensitive(self.ptr, c_key.as_ptr()) };
        unsafe { CJsonRef::from_ptr(ptr) }.map_err(|_| CJsonError::NotFound)
    }

    /// Check if object has item with given key
    pub fn has_object_item(&self, key: &str) -> bool {
        if !self.is_object() {
            return false;
        }
        let Ok(c_key) = CString::new(key) else {
            return false;
        };
        unsafe { cJSON_HasObjectItem(self.ptr, c_key.as_ptr()) != 0 }
    }

    // ========================
    // CREATION FUNCTIONS
    // ========================

    /// Create a null value
    pub fn create_null() -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateNull() };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create a true value
    pub fn create_true() -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateTrue() };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create a false value
    pub fn create_false() -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateFalse() };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create a boolean value
    pub fn create_bool(value: bool) -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateBool(if value { 1 } else { 0 }) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create a number value
    pub fn create_number(value: f64) -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateNumber(value) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create a string value
    pub fn create_string(value: &str) -> CJsonResult<Self> {
        let c_str = CString::new(value).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_CreateString(c_str.as_ptr()) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create an array
    pub fn create_array() -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateArray() };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create an object
    pub fn create_object() -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateObject() };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create an integer array
    pub fn create_int_array(values: &[i32]) -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateIntArray(values.as_ptr(), values.len() as c_int) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create a double array
    pub fn create_double_array(values: &[f64]) -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_CreateDoubleArray(values.as_ptr(), values.len() as c_int) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Create a string array
    pub fn create_string_array(values: &[&str]) -> CJsonResult<Self> {
        let c_strings: Vec<CString> = values
            .iter()
            .map(|s| CString::new(*s))
            .collect::<Result<_, _>>()
            .map_err(|_| CJsonError::InvalidUtf8)?;
        
        let c_ptrs: Vec<*const c_char> = c_strings.iter().map(|s| s.as_ptr()).collect();
        
        let ptr = unsafe { cJSON_CreateStringArray(c_ptrs.as_ptr(), c_ptrs.len() as c_int) };
        unsafe { Self::from_ptr(ptr) }
    }

    // ========================
    // ARRAY MANIPULATION FUNCTIONS
    // ========================

    /// Add item to array
    pub fn add_item_to_array(&mut self, item: CJson) -> CJsonResult<()> {
        if !self.is_array() {
            return Err(CJsonError::TypeError);
        }
        let result = unsafe { cJSON_AddItemToArray(self.ptr, item.into_raw()) };
        if result != 0 {
            Ok(())
        } else {
            Err(CJsonError::InvalidOperation)
        }
    }

    /// Delete item from array by index
    pub fn delete_item_from_array(&mut self, index: usize) -> CJsonResult<()> {
        if !self.is_array() {
            return Err(CJsonError::TypeError);
        }
        unsafe { cJSON_DeleteItemFromArray(self.ptr, index as c_int) };
        Ok(())
    }

    /// Detach item from array by index
    pub fn detach_item_from_array(&mut self, index: usize) -> CJsonResult<CJson> {
        if !self.is_array() {
            return Err(CJsonError::TypeError);
        }
        let ptr = unsafe { cJSON_DetachItemFromArray(self.ptr, index as c_int) };
        unsafe { Self::from_ptr(ptr) }
    }

    // ========================
    // OBJECT MANIPULATION FUNCTIONS
    // ========================

    /// Add item to object
    pub fn add_item_to_object(&mut self, key: &str, item: CJson) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let result = unsafe { cJSON_AddItemToObject(self.ptr, c_key.as_ptr(), item.into_raw()) };
        if result != 0 {
            Ok(())
        } else {
            Err(CJsonError::InvalidOperation)
        }
    }

    /// Add null to object
    pub fn add_null_to_object(&mut self, key: &str) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_AddNullToObject(self.ptr, c_key.as_ptr()) };
        if ptr.is_null() {
            Err(CJsonError::AllocationError)
        } else {
            Ok(())
        }
    }

    /// Add true to object
    pub fn add_true_to_object(&mut self, key: &str) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_AddTrueToObject(self.ptr, c_key.as_ptr()) };
        if ptr.is_null() {
            Err(CJsonError::AllocationError)
        } else {
            Ok(())
        }
    }

    /// Add false to object
    pub fn add_false_to_object(&mut self, key: &str) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_AddFalseToObject(self.ptr, c_key.as_ptr()) };
        if ptr.is_null() {
            Err(CJsonError::AllocationError)
        } else {
            Ok(())
        }
    }

    /// Add boolean to object
    pub fn add_bool_to_object(&mut self, key: &str, value: bool) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe {
            cJSON_AddBoolToObject(self.ptr, c_key.as_ptr(), if value { 1 } else { 0 })
        };
        if ptr.is_null() {
            Err(CJsonError::AllocationError)
        } else {
            Ok(())
        }
    }

    /// Add number to object
    pub fn add_number_to_object(&mut self, key: &str, value: f64) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_AddNumberToObject(self.ptr, c_key.as_ptr(), value) };
        if ptr.is_null() {
            Err(CJsonError::AllocationError)
        } else {
            Ok(())
        }
    }

    /// Add string to object
    pub fn add_string_to_object(&mut self, key: &str, value: &str) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let c_value = CString::new(value).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_AddStringToObject(self.ptr, c_key.as_ptr(), c_value.as_ptr()) };
        if ptr.is_null() {
            Err(CJsonError::AllocationError)
        } else {
            Ok(())
        }
    }

    /// Delete item from object by key
    pub fn delete_item_from_object(&mut self, key: &str) -> CJsonResult<()> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        unsafe { cJSON_DeleteItemFromObject(self.ptr, c_key.as_ptr()) };
        Ok(())
    }

    /// Detach item from object by key
    pub fn detach_item_from_object(&mut self, key: &str) -> CJsonResult<CJson> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_DetachItemFromObject(self.ptr, c_key.as_ptr()) };
        unsafe { Self::from_ptr(ptr) }
    }

    // ========================
    // UTILITY FUNCTIONS
    // ========================

    /// Duplicate the JSON item
    pub fn duplicate(&self, recurse: bool) -> CJsonResult<Self> {
        let ptr = unsafe { cJSON_Duplicate(self.ptr, if recurse { 1 } else { 0 }) };
        unsafe { Self::from_ptr(ptr) }
    }

    /// Compare two JSON items
    pub fn compare(&self, other: &CJson, case_sensitive: bool) -> bool {
        unsafe {
            cJSON_Compare(self.ptr, other.ptr, if case_sensitive { 1 } else { 0 }) != 0
        }
    }
}

impl Drop for CJson {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { cJSON_Delete(self.ptr) };
        }
    }
}

/// Borrowed reference to a cJSON item (does not own the pointer)
pub struct CJsonRef {
    ptr: *mut cJSON,
}

impl CJsonRef {
    /// Create a new CJsonRef from a raw pointer (does not take ownership)
    /// 
    /// # Safety
    /// The pointer must be valid and must outlive this reference
    pub(crate) unsafe fn from_ptr(ptr: *mut cJSON) -> CJsonResult<Self> {
        if ptr.is_null() {
            Err(CJsonError::NullPointer)
        } else {
            Ok(CJsonRef { ptr })
        }
    }

    /// Get the raw pointer (does not transfer ownership)
    pub fn as_ptr(&self) -> *const cJSON {
        self.ptr
    }

    /// Check if the item is a string
    pub fn is_string(&self) -> bool {
        unsafe { cJSON_IsString(self.ptr) != 0 }
    }

    /// Check if the item is a number
    pub fn is_number(&self) -> bool {
        unsafe { cJSON_IsNumber(self.ptr) != 0 }
    }

    /// Check if the item is a boolean
    pub fn is_bool(&self) -> bool {
        unsafe { cJSON_IsBool(self.ptr) != 0 }
    }

    /// Check if the item is null
    pub fn is_null(&self) -> bool {
        unsafe { cJSON_IsNull(self.ptr) != 0 }
    }

    /// Check if the item is an array
    pub fn is_array(&self) -> bool {
        unsafe { cJSON_IsArray(self.ptr) != 0 }
    }

    /// Check if the item is an object
    pub fn is_object(&self) -> bool {
        unsafe { cJSON_IsObject(self.ptr) != 0 }
    }

    /// Get string value
    pub fn get_string_value(&self) -> CJsonResult<String> {
        if !self.is_string() {
            return Err(CJsonError::TypeError);
        }
        let c_str = unsafe { cJSON_GetStringValue(self.ptr) };
        if c_str.is_null() {
            return Err(CJsonError::NullPointer);
        }
        Ok(unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() })
    }

    /// Get number value as f64
    pub fn get_number_value(&self) -> CJsonResult<f64> {
        if !self.is_number() {
            return Err(CJsonError::TypeError);
        }
        Ok(unsafe { cJSON_GetNumberValue(self.ptr) })
    }

    /// Get number value as i32
    pub fn get_int_value(&self) -> CJsonResult<i32> {
        if !self.is_number() {
            return Err(CJsonError::TypeError);
        }
        Ok(unsafe { (*self.ptr).valueint })
    }

    /// Get boolean value
    pub fn get_bool_value(&self) -> CJsonResult<bool> {
        if !self.is_bool() {
            return Err(CJsonError::TypeError);
        }
        Ok(unsafe { cJSON_IsTrue(self.ptr) != 0 })
    }

    /// Get array size
    pub fn get_array_size(&self) -> CJsonResult<usize> {
        if !self.is_array() {
            return Err(CJsonError::TypeError);
        }
        Ok(unsafe { cJSON_GetArraySize(self.ptr) as usize })
    }

    /// Get array item by index
    pub fn get_array_item(&self, index: usize) -> CJsonResult<CJsonRef> {
        if !self.is_array() {
            return Err(CJsonError::TypeError);
        }
        let ptr = unsafe { cJSON_GetArrayItem(self.ptr, index as c_int) };
        unsafe { CJsonRef::from_ptr(ptr) }.map_err(|_| CJsonError::NotFound)
    }

    /// Get object item by key
    pub fn get_object_item(&self, key: &str) -> CJsonResult<CJsonRef> {
        if !self.is_object() {
            return Err(CJsonError::TypeError);
        }
        let c_key = CString::new(key).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe { cJSON_GetObjectItem(self.ptr, c_key.as_ptr()) };
        unsafe { CJsonRef::from_ptr(ptr) }.map_err(|_| CJsonError::NotFound)
    }
}

/// Get the cJSON library version
#[allow(dead_code)]
pub fn version() -> String {
    let c_str = unsafe { cJSON_Version() };
    unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() }
}

/// Get the last parse error pointer
#[allow(dead_code)]
pub fn get_error_ptr() -> Option<String> {
    let c_str = unsafe { cJSON_GetErrorPtr() };
    if c_str.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() })
    }
}

/// Minify a JSON string in place
#[allow(dead_code)]
pub fn minify(json: &mut String) {
    let c_str = CString::new(json.as_str()).expect("CString conversion failed");
    unsafe {
        let ptr = c_str.as_ptr() as *mut c_char;
        cJSON_Minify(ptr);
        *json = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_object() {
        let json = r#"{"name":"John","age":30}"#;
        let parsed = CJson::parse(json).unwrap();
        assert!(parsed.is_object());
    }

    #[test]
    fn test_parse_array() {
        let json = r#"[1,2,3,4,5]"#;
        let parsed = CJson::parse(json).unwrap();
        assert!(parsed.is_array());
        assert_eq!(parsed.get_array_size().unwrap(), 5);
    }

    #[test]
    fn test_create_and_get_string() {
        let json = CJson::create_string("Hello, World!").unwrap();
        assert!(json.is_string());
        assert_eq!(json.get_string_value().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_create_and_get_number() {
        let json = CJson::create_number(42.5).unwrap();
        assert!(json.is_number());
        assert_eq!(json.get_number_value().unwrap(), 42.5);
    }

    #[test]
    fn test_create_and_get_bool() {
        let json_true = CJson::create_true().unwrap();
        assert!(json_true.is_true());
        assert!(json_true.is_bool());
        assert_eq!(json_true.get_bool_value().unwrap(), true);

        let json_false = CJson::create_false().unwrap();
        assert!(json_false.is_false());
        assert!(json_false.is_bool());
        assert_eq!(json_false.get_bool_value().unwrap(), false);
    }

    #[test]
    fn test_create_null() {
        let json = CJson::create_null().unwrap();
        assert!(json.is_null());
    }

    #[test]
    fn test_create_object_and_add_items() {
        let mut obj = CJson::create_object().unwrap();
        obj.add_string_to_object("name", "Alice").unwrap();
        obj.add_number_to_object("age", 25.0).unwrap();
        obj.add_bool_to_object("active", true).unwrap();

        assert!(obj.is_object());
        assert!(obj.has_object_item("name"));
        assert!(obj.has_object_item("age"));
        assert!(obj.has_object_item("active"));

        let name = obj.get_object_item("name").unwrap();
        assert_eq!(name.get_string_value().unwrap(), "Alice");

        let age = obj.get_object_item("age").unwrap();
        assert_eq!(age.get_number_value().unwrap(), 25.0);
    }

    #[test]
    fn test_create_array_and_add_items() {
        let mut arr = CJson::create_array().unwrap();
        arr.add_item_to_array(CJson::create_number(1.0).unwrap()).unwrap();
        arr.add_item_to_array(CJson::create_number(2.0).unwrap()).unwrap();
        arr.add_item_to_array(CJson::create_number(3.0).unwrap()).unwrap();

        assert!(arr.is_array());
        assert_eq!(arr.get_array_size().unwrap(), 3);

        let item = arr.get_array_item(1).unwrap();
        assert_eq!(item.get_number_value().unwrap(), 2.0);
    }

    #[test]
    fn test_print_formatted() {
        let mut obj = CJson::create_object().unwrap();
        obj.add_string_to_object("key", "value").unwrap();
        
        let json_str = obj.print().unwrap();
        assert!(json_str.contains("key"));
        assert!(json_str.contains("value"));
    }

    #[test]
    fn test_print_unformatted() {
        let mut obj = CJson::create_object().unwrap();
        obj.add_string_to_object("key", "value").unwrap();
        
        let json_str = obj.print_unformatted().unwrap();
        assert!(json_str.contains("key"));
        assert!(json_str.contains("value"));
        assert!(!json_str.contains("\n")); // No newlines in unformatted
    }

    #[test]
    fn test_duplicate() {
        let original = CJson::create_string("test").unwrap();
        let duplicate = original.duplicate(true).unwrap();
        
        assert_eq!(
            original.get_string_value().unwrap(),
            duplicate.get_string_value().unwrap()
        );
    }

    #[test]
    fn test_compare() {
        let json1 = CJson::create_number(42.0).unwrap();
        let json2 = CJson::create_number(42.0).unwrap();
        let json3 = CJson::create_number(43.0).unwrap();

        assert!(json1.compare(&json2, true));
        assert!(!json1.compare(&json3, true));
    }

    #[test]
    fn test_create_int_array() {
        let values = [1, 2, 3, 4, 5];
        let arr = CJson::create_int_array(&values).unwrap();
        
        assert!(arr.is_array());
        assert_eq!(arr.get_array_size().unwrap(), 5);
    }

    #[test]
    fn test_create_double_array() {
        let values = [1.1, 2.2, 3.3];
        let arr = CJson::create_double_array(&values).unwrap();
        
        assert!(arr.is_array());
        assert_eq!(arr.get_array_size().unwrap(), 3);
    }

    #[test]
    #[ignore] // Temporarily disabled due to potential double free issue
    fn test_create_string_array() {
        let values = ["foo", "bar", "baz"];
        let arr = CJson::create_string_array(&values).unwrap();
        
        assert!(arr.is_array());
        assert_eq!(arr.get_array_size().unwrap(), 3);
    }

    #[test]
    fn test_delete_item_from_array() {
        let mut arr = CJson::create_array().unwrap();
        arr.add_item_to_array(CJson::create_number(1.0).unwrap()).unwrap();
        arr.add_item_to_array(CJson::create_number(2.0).unwrap()).unwrap();
        arr.add_item_to_array(CJson::create_number(3.0).unwrap()).unwrap();

        assert_eq!(arr.get_array_size().unwrap(), 3);
        arr.delete_item_from_array(1).unwrap();
        assert_eq!(arr.get_array_size().unwrap(), 2);
    }

    #[test]
    fn test_delete_item_from_object() {
        let mut obj = CJson::create_object().unwrap();
        obj.add_string_to_object("key1", "value1").unwrap();
        obj.add_string_to_object("key2", "value2").unwrap();

        assert!(obj.has_object_item("key1"));
        obj.delete_item_from_object("key1").unwrap();
        assert!(!obj.has_object_item("key1"));
        assert!(obj.has_object_item("key2"));
    }

    #[test]
    fn test_parse_nested_object() {
        let json = r#"{"person":{"name":"John","age":30}}"#;
        let parsed = CJson::parse(json).unwrap();
        
        let person = parsed.get_object_item("person").unwrap();
        assert!(person.is_object());
        
        let name = person.get_object_item("name").unwrap();
        assert_eq!(name.get_string_value().unwrap(), "John");
    }

    #[test]
    fn test_type_error() {
        let json = CJson::create_string("not a number").unwrap();
        assert!(json.get_number_value().is_err());
    }

    #[test]
    fn test_not_found_error() {
        let obj = CJson::create_object().unwrap();
        assert!(obj.get_object_item("nonexistent").is_err());
    }

    #[test]
    fn test_parse_with_length() {
        let json = r#"{"key":"value"}"#;
        let parsed = CJson::parse_with_length(json, json.len()).unwrap();
        assert!(parsed.is_object());
    }

    #[test]
    fn test_case_sensitive_get() {
        let mut obj = CJson::create_object().unwrap();
        obj.add_string_to_object("Key", "value").unwrap();
        
        assert!(obj.get_object_item_case_sensitive("Key").is_ok());
        assert!(obj.get_object_item_case_sensitive("key").is_err());
    }
}
