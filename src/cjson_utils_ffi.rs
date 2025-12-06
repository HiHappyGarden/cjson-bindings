#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clashing_extern_declarations)]


use crate::cjson_ffi::{ cJSON };

// ========================
// EXTERN "C" FUNCTION DECLARATIONS
// ========================

unsafe extern "C" {
    // RFC6901: JSON Pointer
    /// Get a pointer to a value in a JSON object using RFC6901 syntax.
    /// Returns NULL if not found or invalid.
    pub fn cJSONUtils_GetPointer(object: *mut cJSON, pointer: *const i8) -> *mut cJSON;

    /// Get a pointer to a value in a JSON object using RFC6901 syntax, case-sensitive.
    pub fn cJSONUtils_GetPointerCaseSensitive(object: *mut cJSON, pointer: *const i8) -> *mut cJSON;

    // RFC6902: JSON Patch
    /// Generate a JSON Patch (RFC6902) to transform 'from' into 'to'.
    /// Modifies both 'from' and 'to' by sorting keys.
    pub fn cJSONUtils_GeneratePatches(from: *mut cJSON, to: *mut cJSON) -> *mut cJSON;

    /// Generate a JSON Patch (RFC6902) to transform 'from' into 'to', case-sensitive.
    pub fn cJSONUtils_GeneratePatchesCaseSensitive(from: *mut cJSON, to: *mut cJSON) -> *mut cJSON;

    /// Add a patch operation to an array.
    pub fn cJSONUtils_AddPatchToArray(array: *mut cJSON, operation: *const i8, path: *const i8, value: *const cJSON);

    /// Apply a JSON Patch (RFC6902) to an object.
    /// Returns 0 on success.
    pub fn cJSONUtils_ApplyPatches(object: *mut cJSON, patches: *const cJSON) -> i32;

    /// Apply a JSON Patch (RFC6902) to an object, case-sensitive.
    pub fn cJSONUtils_ApplyPatchesCaseSensitive(object: *mut cJSON, patches: *const cJSON) -> i32;

    // RFC7386: JSON Merge Patch
    /// Apply a JSON Merge Patch (RFC7386) to 'target'.
    /// Returns new pointer to modified 'target'.
    pub fn cJSONUtils_MergePatch(target: *mut cJSON, patch: *const cJSON) -> *mut cJSON;

    /// Apply a JSON Merge Patch (RFC7386) to 'target', case-sensitive.
    pub fn cJSONUtils_MergePatchCaseSensitive(target: *mut cJSON, patch: *const cJSON) -> *mut cJSON;

    /// Generate a JSON Merge Patch to transform 'from' into 'to'.
    /// Modifies both 'from' and 'to' by sorting keys.
    pub fn cJSONUtils_GenerateMergePatch(from: *mut cJSON, to: *mut cJSON) -> *mut cJSON;

    /// Generate a JSON Merge Patch to transform 'from' into 'to', case-sensitive.
    pub fn cJSONUtils_GenerateMergePatchCaseSensitive(from: *mut cJSON, to: *mut cJSON) -> *mut cJSON;

    // Utility: Find pointer from object to target
    /// Find a JSON Pointer from 'object' to 'target'.
    /// Returns NULL if not found.
    pub fn cJSONUtils_FindPointerFromObjectTo(object: *const cJSON, target: *const cJSON) -> *mut i8;

    // Sort object members alphabetically
    /// Sort object members alphabetically (case-insensitive).
    pub fn cJSONUtils_SortObject(object: *mut cJSON);

    /// Sort object members alphabetically (case-sensitive).
    pub fn cJSONUtils_SortObjectCaseSensitive(object: *mut cJSON);
}