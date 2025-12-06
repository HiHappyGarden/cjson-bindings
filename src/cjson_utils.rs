//! Safe Rust wrappers for cJSON_Utils library
//!
//! This module provides safe, idiomatic Rust interfaces over the cJSON_Utils C library,
//! which implements RFC6901 (JSON Pointer), RFC6902 (JSON Patch), and RFC7386 (JSON Merge Patch).

extern crate alloc;

use alloc::ffi::CString;
use alloc::string::String;
use core::ffi::{CStr, c_char};

use crate::cjson::{CJson, CJsonError, CJsonResult};
use crate::cjson_ffi::cJSON;
use crate::cjson_utils_ffi::*;

/// JSON Pointer utilities (RFC6901)
pub struct JsonPointer;

impl JsonPointer {
    /// Get a value from a JSON object using RFC6901 JSON Pointer syntax.
    /// 
    /// # Arguments
    /// * `object` - The JSON object to search in
    /// * `pointer` - The JSON Pointer string (e.g., "/foo/bar/0")
    /// 
    /// # Returns
    /// A borrowed reference to the found item, or NotFound error
    pub fn get(object: &CJson, pointer: &str) -> CJsonResult<CJsonRef> {
        let c_pointer = CString::new(pointer).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe {
            cJSONUtils_GetPointer(object.as_ptr() as *mut cJSON, c_pointer.as_ptr() as *const i8)
        };
        unsafe { CJsonRef::from_ptr(ptr) }.map_err(|_| CJsonError::NotFound)
    }

    /// Get a value from a JSON object using RFC6901 JSON Pointer syntax (case-sensitive).
    /// 
    /// # Arguments
    /// * `object` - The JSON object to search in
    /// * `pointer` - The JSON Pointer string (e.g., "/foo/bar/0")
    /// 
    /// # Returns
    /// A borrowed reference to the found item, or NotFound error
    pub fn get_case_sensitive(object: &CJson, pointer: &str) -> CJsonResult<CJsonRef> {
        let c_pointer = CString::new(pointer).map_err(|_| CJsonError::InvalidUtf8)?;
        let ptr = unsafe {
            cJSONUtils_GetPointerCaseSensitive(
                object.as_ptr() as *mut cJSON,
                c_pointer.as_ptr() as *const i8,
            )
        };
        unsafe { CJsonRef::from_ptr(ptr) }.map_err(|_| CJsonError::NotFound)
    }

    /// Find a JSON Pointer path from one object to a target value within it.
    /// 
    /// # Arguments
    /// * `object` - The JSON object to search in
    /// * `target` - The target value to find
    /// 
    /// # Returns
    /// The JSON Pointer path as a String, or NotFound error
    pub fn find_from_object_to(object: &CJson, target: &CJson) -> CJsonResult<String> {
        let ptr = unsafe {
            cJSONUtils_FindPointerFromObjectTo(object.as_ptr(), target.as_ptr())
        };
        if ptr.is_null() {
            return Err(CJsonError::NotFound);
        }
        let path = unsafe { CStr::from_ptr(ptr as *const c_char).to_string_lossy().into_owned() };
        unsafe { crate::cjson_ffi::cJSON_free(ptr as *mut core::ffi::c_void) };
        Ok(path)
    }
}

/// JSON Patch utilities (RFC6902)
pub struct JsonPatch;

impl JsonPatch {
    /// Generate a JSON Patch (RFC6902) to transform 'from' into 'to'.
    /// 
    /// Note: This function modifies both 'from' and 'to' by sorting their keys.
    /// 
    /// # Arguments
    /// * `from` - The original JSON object
    /// * `to` - The target JSON object
    /// 
    /// # Returns
    /// A new CJson object containing the patch operations
    pub fn generate(from: &mut CJson, to: &mut CJson) -> CJsonResult<CJson> {
        let ptr = unsafe {
            cJSONUtils_GeneratePatches(from.as_mut_ptr(), to.as_mut_ptr())
        };
        unsafe { CJson::from_ptr(ptr) }
    }

    /// Generate a JSON Patch (RFC6902) to transform 'from' into 'to' (case-sensitive).
    /// 
    /// Note: This function modifies both 'from' and 'to' by sorting their keys.
    /// 
    /// # Arguments
    /// * `from` - The original JSON object
    /// * `to` - The target JSON object
    /// 
    /// # Returns
    /// A new CJson object containing the patch operations
    pub fn generate_case_sensitive(from: &mut CJson, to: &mut CJson) -> CJsonResult<CJson> {
        let ptr = unsafe {
            cJSONUtils_GeneratePatchesCaseSensitive(from.as_mut_ptr(), to.as_mut_ptr())
        };
        unsafe { CJson::from_ptr(ptr) }
    }

    /// Apply a JSON Patch (RFC6902) to an object.
    /// 
    /// # Arguments
    /// * `object` - The JSON object to patch
    /// * `patches` - The patch operations to apply
    /// 
    /// # Returns
    /// Ok(()) on success, or an error
    pub fn apply(object: &mut CJson, patches: &CJson) -> CJsonResult<()> {
        let result = unsafe {
            cJSONUtils_ApplyPatches(object.as_mut_ptr(), patches.as_ptr())
        };
        if result == 0 {
            Ok(())
        } else {
            Err(CJsonError::InvalidOperation)
        }
    }

    /// Apply a JSON Patch (RFC6902) to an object (case-sensitive).
    /// 
    /// # Arguments
    /// * `object` - The JSON object to patch
    /// * `patches` - The patch operations to apply
    /// 
    /// # Returns
    /// Ok(()) on success, or an error
    pub fn apply_case_sensitive(object: &mut CJson, patches: &CJson) -> CJsonResult<()> {
        let result = unsafe {
            cJSONUtils_ApplyPatchesCaseSensitive(object.as_mut_ptr(), patches.as_ptr())
        };
        if result == 0 {
            Ok(())
        } else {
            Err(CJsonError::InvalidOperation)
        }
    }

    /// Add a patch operation to a patches array.
    /// 
    /// # Arguments
    /// * `array` - The array of patch operations
    /// * `operation` - The operation type ("add", "remove", "replace", "move", "copy", "test")
    /// * `path` - The JSON Pointer path
    /// * `value` - The value for the operation (optional for some operations)
    pub fn add_to_array(
        array: &mut CJson,
        operation: &str,
        path: &str,
        value: Option<&CJson>,
    ) -> CJsonResult<()> {
        if !array.is_array() {
            return Err(CJsonError::TypeError);
        }

        let c_operation = CString::new(operation).map_err(|_| CJsonError::InvalidUtf8)?;
        let c_path = CString::new(path).map_err(|_| CJsonError::InvalidUtf8)?;

        let value_ptr = value.map(|v| v.as_ptr()).unwrap_or(core::ptr::null());

        unsafe {
            cJSONUtils_AddPatchToArray(
                array.as_mut_ptr(),
                c_operation.as_ptr() as *const i8,
                c_path.as_ptr() as *const i8,
                value_ptr,
            );
        }
        Ok(())
    }
}

/// JSON Merge Patch utilities (RFC7386)
pub struct JsonMergePatch;

impl JsonMergePatch {
    /// Apply a JSON Merge Patch (RFC7386) to a target object.
    /// 
    /// # Arguments
    /// * `target` - The JSON object to merge into
    /// * `patch` - The merge patch to apply
    /// 
    /// # Returns
    /// A new CJson object with the merged result
    pub fn apply(target: &mut CJson, patch: &CJson) -> CJsonResult<CJson> {
        let ptr = unsafe {
            cJSONUtils_MergePatch(target.as_mut_ptr(), patch.as_ptr())
        };
        unsafe { CJson::from_ptr(ptr) }
    }

    /// Apply a JSON Merge Patch (RFC7386) to a target object (case-sensitive).
    /// 
    /// # Arguments
    /// * `target` - The JSON object to merge into
    /// * `patch` - The merge patch to apply
    /// 
    /// # Returns
    /// A new CJson object with the merged result
    pub fn apply_case_sensitive(target: &mut CJson, patch: &CJson) -> CJsonResult<CJson> {
        let ptr = unsafe {
            cJSONUtils_MergePatchCaseSensitive(target.as_mut_ptr(), patch.as_ptr())
        };
        unsafe { CJson::from_ptr(ptr) }
    }

    /// Generate a JSON Merge Patch to transform 'from' into 'to'.
    /// 
    /// Note: This function modifies both 'from' and 'to' by sorting their keys.
    /// 
    /// # Arguments
    /// * `from` - The original JSON object
    /// * `to` - The target JSON object
    /// 
    /// # Returns
    /// A new CJson object containing the merge patch
    pub fn generate(from: &mut CJson, to: &mut CJson) -> CJsonResult<CJson> {
        let ptr = unsafe {
            cJSONUtils_GenerateMergePatch(from.as_mut_ptr(), to.as_mut_ptr())
        };
        unsafe { CJson::from_ptr(ptr) }
    }

    /// Generate a JSON Merge Patch to transform 'from' into 'to' (case-sensitive).
    /// 
    /// Note: This function modifies both 'from' and 'to' by sorting their keys.
    /// 
    /// # Arguments
    /// * `from` - The original JSON object
    /// * `to` - The target JSON object
    /// 
    /// # Returns
    /// A new CJson object containing the merge patch
    pub fn generate_case_sensitive(from: &mut CJson, to: &mut CJson) -> CJsonResult<CJson> {
        let ptr = unsafe {
            cJSONUtils_GenerateMergePatchCaseSensitive(from.as_mut_ptr(), to.as_mut_ptr())
        };
        unsafe { CJson::from_ptr(ptr) }
    }
}

/// Utility functions for JSON object manipulation
pub struct JsonUtils;

impl JsonUtils {
    /// Sort object members alphabetically (case-insensitive).
    /// 
    /// # Arguments
    /// * `object` - The JSON object to sort
    pub fn sort_object(object: &mut CJson) -> CJsonResult<()> {
        if !object.is_object() {
            return Err(CJsonError::TypeError);
        }
        unsafe { cJSONUtils_SortObject(object.as_mut_ptr()) };
        Ok(())
    }

    /// Sort object members alphabetically (case-sensitive).
    /// 
    /// # Arguments
    /// * `object` - The JSON object to sort
    pub fn sort_object_case_sensitive(object: &mut CJson) -> CJsonResult<()> {
        if !object.is_object() {
            return Err(CJsonError::TypeError);
        }
        unsafe { cJSONUtils_SortObjectCaseSensitive(object.as_mut_ptr()) };
        Ok(())
    }
}

/// Re-export CJsonRef for use with pointer operations
pub use crate::cjson::CJsonRef;
