#![no_std]


pub(crate) mod cjson_ffi;
mod cjson;

pub(crate) mod cjson_utils_ffi;
mod cjson_utils;

// Re-export main types for convenience
pub use cjson::{CJson, CJsonRef, CJsonResult, CJsonError};
pub use cjson_utils::{JsonPointer, JsonPatch, JsonMergePatch, JsonUtils};

