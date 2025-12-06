#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clashing_extern_declarations)]

use core::ffi::{c_char, c_double, c_int, c_void};

// ========================
// COSTANTI
// ========================

pub const cJSON_Invalid: c_int = 0;
pub const cJSON_False: c_int = 1 << 0;
pub const cJSON_True: c_int = 1 << 1;
pub const cJSON_NULL: c_int = 1 << 2;
pub const cJSON_Number: c_int = 1 << 3;
pub const cJSON_String: c_int = 1 << 4;
pub const cJSON_Array: c_int = 1 << 5;
pub const cJSON_Object: c_int = 1 << 6;
pub const cJSON_Raw: c_int = 1 << 7;

pub const cJSON_IsReference: c_int = 256;
pub const cJSON_StringIsConst: c_int = 512;

pub const CJSON_VERSION_MAJOR: c_int = 1;
pub const CJSON_VERSION_MINOR: c_int = 7;
pub const CJSON_VERSION_PATCH: c_int = 19;

pub const CJSON_NESTING_LIMIT: c_int = 1000;
pub const CJSON_CIRCULAR_LIMIT: c_int = 10000;

// ========================
// TYPES
// ========================

// Struct cJSON
#[repr(C)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub type_: c_int, // "type" it is a reserved keyword in Rust
    pub valuestring: *mut c_char,
    pub valueint: c_int,
    pub valuedouble: c_double,
    pub string: *mut c_char,
}

pub type cJSON_bool = c_int;

// cJSON_Hooks struct
#[repr(C)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option<unsafe extern "C" fn(size: usize) -> *mut c_void>,
    pub free_fn: Option<unsafe extern "C" fn(ptr: *mut c_void)>,
}

// ========================
// FUNCTIONS C (declare extern "C")
// ========================

unsafe extern "C" {
    pub fn cJSON_Version() -> *const c_char;

    pub fn cJSON_InitHooks(hooks: *mut cJSON_Hooks);

    pub fn cJSON_Parse(value: *const c_char) -> *mut cJSON;
    pub fn cJSON_ParseWithLength(value: *const c_char, buffer_length: usize) -> *mut cJSON;
    pub fn cJSON_ParseWithOpts(
        value: *const c_char,
        return_parse_end: *mut *const c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;
    pub fn cJSON_ParseWithLengthOpts(
        value: *const c_char,
        buffer_length: usize,
        return_parse_end: *mut *const c_char,
        require_null_terminated: cJSON_bool,
    ) -> *mut cJSON;

    pub fn cJSON_Print(item: *const cJSON) -> *mut c_char;
    pub fn cJSON_PrintUnformatted(item: *const cJSON) -> *mut c_char;
    pub fn cJSON_PrintBuffered(item: *const cJSON, prebuffer: c_int, fmt: cJSON_bool) -> *mut c_char;
    pub fn cJSON_PrintPreallocated(
        item: *mut cJSON,
        buffer: *mut c_char,
        length: c_int,
        format: cJSON_bool,
    ) -> cJSON_bool;

    pub fn cJSON_Delete(item: *mut cJSON);

    pub fn cJSON_GetArraySize(array: *const cJSON) -> c_int;
    pub fn cJSON_GetArrayItem(array: *const cJSON, index: c_int) -> *mut cJSON;
    pub fn cJSON_GetObjectItem(object: *const cJSON, string: *const c_char) -> *mut cJSON;
    pub fn cJSON_GetObjectItemCaseSensitive(object: *const cJSON, string: *const c_char) -> *mut cJSON;
    pub fn cJSON_HasObjectItem(object: *const cJSON, string: *const c_char) -> cJSON_bool;

    pub fn cJSON_GetErrorPtr() -> *const c_char;

    pub fn cJSON_GetStringValue(item: *const cJSON) -> *const c_char;
    pub fn cJSON_GetNumberValue(item: *const cJSON) -> c_double;

    pub fn cJSON_IsInvalid(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsFalse(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsTrue(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsBool(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsNull(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsNumber(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsString(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsArray(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsObject(item: *const cJSON) -> cJSON_bool;
    pub fn cJSON_IsRaw(item: *const cJSON) -> cJSON_bool;

    pub fn cJSON_CreateNull() -> *mut cJSON;
    pub fn cJSON_CreateTrue() -> *mut cJSON;
    pub fn cJSON_CreateFalse() -> *mut cJSON;
    pub fn cJSON_CreateBool(boolean: cJSON_bool) -> *mut cJSON;
    pub fn cJSON_CreateNumber(num: c_double) -> *mut cJSON;
    pub fn cJSON_CreateString(string: *const c_char) -> *mut cJSON;
    pub fn cJSON_CreateRaw(raw: *const c_char) -> *mut cJSON;
    pub fn cJSON_CreateArray() -> *mut cJSON;
    pub fn cJSON_CreateObject() -> *mut cJSON;

    pub fn cJSON_CreateStringReference(string: *const c_char) -> *mut cJSON;
    pub fn cJSON_CreateObjectReference(child: *const cJSON) -> *mut cJSON;
    pub fn cJSON_CreateArrayReference(child: *const cJSON) -> *mut cJSON;

    pub fn cJSON_CreateIntArray(numbers: *const c_int, count: c_int) -> *mut cJSON;
    pub fn cJSON_CreateFloatArray(numbers: *const f32, count: c_int) -> *mut cJSON;
    pub fn cJSON_CreateDoubleArray(numbers: *const c_double, count: c_int) -> *mut cJSON;
    pub fn cJSON_CreateStringArray(strings: *const *const c_char, count: c_int) -> *mut cJSON;

    pub fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_AddItemToObject(object: *mut cJSON, string: *const c_char, item: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_AddItemToObjectCS(object: *mut cJSON, string: *const c_char, item: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_AddItemReferenceToArray(array: *mut cJSON, item: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_AddItemReferenceToObject(object: *mut cJSON, string: *const c_char, item: *mut cJSON) -> cJSON_bool;

    pub fn cJSON_DetachItemViaPointer(parent: *mut cJSON, item: *mut cJSON) -> *mut cJSON;
    pub fn cJSON_DetachItemFromArray(array: *mut cJSON, which: c_int) -> *mut cJSON;
    pub fn cJSON_DeleteItemFromArray(array: *mut cJSON, which: c_int);
    pub fn cJSON_DetachItemFromObject(object: *mut cJSON, string: *const c_char) -> *mut cJSON;
    pub fn cJSON_DetachItemFromObjectCaseSensitive(object: *mut cJSON, string: *const c_char) -> *mut cJSON;
    pub fn cJSON_DeleteItemFromObject(object: *mut cJSON, string: *const c_char);
    pub fn cJSON_DeleteItemFromObjectCaseSensitive(object: *mut cJSON, string: *const c_char);

    pub fn cJSON_InsertItemInArray(array: *mut cJSON, which: c_int, newitem: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_ReplaceItemViaPointer(parent: *mut cJSON, item: *mut cJSON, replacement: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_ReplaceItemInArray(array: *mut cJSON, which: c_int, newitem: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_ReplaceItemInObject(object: *mut cJSON, string: *const c_char, newitem: *mut cJSON) -> cJSON_bool;
    pub fn cJSON_ReplaceItemInObjectCaseSensitive(object: *mut cJSON, string: *const c_char, newitem: *mut cJSON) -> cJSON_bool;

    pub fn cJSON_Duplicate(item: *const cJSON, recurse: cJSON_bool) -> *mut cJSON;
    pub fn cJSON_Compare(a: *const cJSON, b: *const cJSON, case_sensitive: cJSON_bool) -> cJSON_bool;

    pub fn cJSON_Minify(json: *mut c_char);

    pub fn cJSON_AddNullToObject(object: *mut cJSON, name: *const c_char) -> *mut cJSON;
    pub fn cJSON_AddTrueToObject(object: *mut cJSON, name: *const c_char) -> *mut cJSON;
    pub fn cJSON_AddFalseToObject(object: *mut cJSON, name: *const c_char) -> *mut cJSON;
    pub fn cJSON_AddBoolToObject(object: *mut cJSON, name: *const c_char, boolean: cJSON_bool) -> *mut cJSON;
    pub fn cJSON_AddNumberToObject(object: *mut cJSON, name: *const c_char, number: c_double) -> *mut cJSON;
    pub fn cJSON_AddStringToObject(object: *mut cJSON, name: *const c_char, string: *const c_char) -> *mut cJSON;
    pub fn cJSON_AddRawToObject(object: *mut cJSON, name: *const c_char, raw: *const c_char) -> *mut cJSON;
    pub fn cJSON_AddObjectToObject(object: *mut cJSON, name: *const c_char) -> *mut cJSON;
    pub fn cJSON_AddArrayToObject(object: *mut cJSON, name: *const c_char) -> *mut cJSON;

    pub fn cJSON_SetNumberHelper(object: *mut cJSON, number: c_double) -> c_double;
    pub fn cJSON_SetValuestring(object: *mut cJSON, valuestring: *const c_char) -> *mut c_char;

    pub fn cJSON_malloc(size: usize) -> *mut c_void;
    pub fn cJSON_free(object: *mut c_void);
}

// ========================
// MACRO (translated from C to Rust)
// ========================

// cJSON_SetIntValue
#[macro_export]
macro_rules! cJSON_SetIntValue {
    ($object:expr, $number:expr) => {
        if let Some(obj) = unsafe { $object.as_mut() } {
            obj.valueint = $number;
            obj.valuedouble = $number as f64;
            $number
        } else {
            $number
        }
    };
}

// cJSON_SetNumberValue
#[macro_export]
macro_rules! cJSON_SetNumberValue {
    ($object:expr, $number:expr) => {
        if let Some(obj) = unsafe { $object.as_mut() } {
            crate::cJSON_SetNumberHelper(obj, $number as f64)
        } else {
            $number as f64
        }
    };
}

// cJSON_SetBoolValue
#[macro_export]
macro_rules! cJSON_SetBoolValue {
    ($object:expr, $boolValue:expr) => {
        if let Some(obj) = unsafe { $object.as_mut() } {
            if (obj.type_ & (crate::cJSON_False | crate::cJSON_True)) != 0 {
                obj.type_ = (obj.type_ & !(crate::cJSON_False | crate::cJSON_True)) | if $boolValue { crate::cJSON_True } else { crate::cJSON_False };
                obj.type_
            } else {
                crate::cJSON_Invalid
            }
        } else {
            crate::cJSON_Invalid
        }
    };
}

// cJSON_ArrayForEach
#[macro_export]
macro_rules! cJSON_ArrayForEach {
    ($element:ident, $array:expr) => {
        let mut $element = if let Some(arr) = unsafe { $array.as_ref() } {
            arr.child
        } else {
            core::ptr::null_mut()
        };
        while !($element).is_null() {
            {
                // body of the loop goes here
            }
            $element = unsafe { (*$element).next };
        }
    };
}

// // ========================
// // FUNZIONI DI UTILITÀ (opzionali)
// // ========================

// // Wrapper sicuro per cJSON_Delete (evita double free)
// pub unsafe fn cJSON_Delete_safe(item: *mut cJSON) {
//     if !item.is_null() {
//         unsafe { cJSON_Delete(item); }
//     }
// }

// // Wrapper per cJSON_Parse con gestione errori — usa alloc::string::String
// pub unsafe fn cJSON_Parse_safe(json_str: &str) -> Option<*mut cJSON> {
//     let c_str = CString::new(json_str).ok()?;
//     let ptr = unsafe { cJSON_Parse(c_str.as_ptr()) };
//     if ptr.is_null() {
//         None
//     } else {
//         Some(ptr)
//     }
// }

// // Wrapper per cJSON_Print con deallocazione automatica — ritorna alloc::string::String
// pub unsafe fn cJSON_Print_safe(item: *mut cJSON) -> Option<String> {
//     if item.is_null() {
//         return None;
//     }
//     let c_str = unsafe { cJSON_Print(item) };
//     if c_str.is_null() {
//         return None;
//     }
//     let rust_str = unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() };
//     unsafe { cJSON_free(c_str as *mut c_void); }
//     Some(rust_str)
// }

// // Wrapper per cJSON_GetStringValue — ritorna String
// pub unsafe fn cJSON_GetStringValue_safe(item: *const cJSON) -> Option<String> {
//     if item.is_null() {
//         return None;
//     }
//     let c_str = unsafe { cJSON_GetStringValue(item) };
//     if c_str.is_null() {
//         return None;
//     }
//     Some(unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() })
// }

// // Wrapper per cJSON_GetNumberValue — ritorna f64
// pub unsafe fn cJSON_GetNumberValue_safe(item: *const cJSON) -> Option<f64> {
//     if item.is_null() {
//         return None;
//     }
//     Some(unsafe { cJSON_GetNumberValue(item) })
// }