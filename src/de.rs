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


use osal_rs_serde::Deserialize;
use osal_rs_serde::Deserializer;
use osal_rs::utils::hex_to_bytes_into_slice;

use crate::CJsonResult;
use crate::cjson::CJsonError;
use crate::cjson::CJson;
use crate::cjson::CJsonRef;
use crate::cjson_ffi::cJSON_Duplicate;
use core::fmt::Write;

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;


pub struct JsonDeserializer {
    stack: BTreeMap<String, CJson>,
    stack_name: Vec<String>,
}

impl Deserializer for JsonDeserializer {
    type Error = CJsonError;

    fn deserialize_bool(&mut self, name: &str) -> core::result::Result<bool, Self::Error> {
        let item = self.get_item(name)?;
        item.get_bool_value()
    }

    fn deserialize_u8(&mut self, name: &str) -> core::result::Result<u8, Self::Error> {
        let v = self.deserialize_u64(name)?;
        if v <= u8::MAX as u64 { Ok(v as u8) } else { Err(CJsonError::TypeError) }
    }

    fn deserialize_i8(&mut self, name: &str) -> core::result::Result<i8, Self::Error> {
        let v = self.deserialize_i64(name)?;
        if v >= i8::MIN as i64 && v <= i8::MAX as i64 { Ok(v as i8) } else { Err(CJsonError::TypeError) }
    }


    fn deserialize_u16(&mut self, name: &str) -> core::result::Result<u16, Self::Error> {
        let v = self.deserialize_u64(name)?;
        if v <= u16::MAX as u64 { Ok(v as u16) } else { Err(CJsonError::TypeError) }
    }

    fn deserialize_i16(&mut self, name: &str) -> core::result::Result<i16, Self::Error> {
        let v = self.deserialize_i64(name)?;
        if v >= i16::MIN as i64 && v <= i16::MAX as i64 { Ok(v as i16) } else { Err(CJsonError::TypeError) }
    }

    fn deserialize_u32(&mut self, name: &str) -> core::result::Result<u32, Self::Error> {
        let v = self.deserialize_u64(name)?;
        if v <= u32::MAX as u64 { Ok(v as u32) } else { Err(CJsonError::TypeError) }
    }

    fn deserialize_i32(&mut self, name: &str) -> core::result::Result<i32, Self::Error> {
        let v = self.deserialize_i64(name)?;
        if v >= i32::MIN as i64 && v <= i32::MAX as i64 { Ok(v as i32) } else { Err(CJsonError::TypeError) }
    }

    fn deserialize_u64(&mut self, name: &str) -> core::result::Result<u64, Self::Error> {
        let item = self.get_item(name)?;
        let n = item.get_number_value()?;
        if n < 0.0 { return Err(CJsonError::TypeError); }
        Ok(n as u64)
    }

    fn deserialize_i64(&mut self, name: &str) -> core::result::Result<i64, Self::Error> {
        let item = self.get_item(name)?;
        let n = item.get_number_value()?;
        Ok(n as i64)
    }

    fn deserialize_u128(&mut self, name: &str) -> core::result::Result<u128, Self::Error> {
        let v = self.deserialize_u64(name)?;
        Ok(v as u128)
    }

    fn deserialize_i128(&mut self, name: &str) -> core::result::Result<i128, Self::Error> {
        let v = self.deserialize_i64(name)?;
        Ok(v as i128)
    }

    fn deserialize_f32(&mut self, name: &str) -> core::result::Result<f32, Self::Error> {
        let item = self.get_item(name)?;
        let n = item.get_number_value()?;
        Ok(n as f32)
    }

    fn deserialize_f64(&mut self, name: &str) -> core::result::Result<f64, Self::Error> {
        let item = self.get_item(name)?;
        item.get_number_value()
    }

    fn deserialize_bytes(&mut self, name: &str, buffer: &mut [u8]) -> core::result::Result<usize, Self::Error> {
        let item = self.get_item(name)?;

        if item.is_string() {
            let s = item.get_string_value()?;
            
            // Check if the string is a hex-encoded string
            // (even length and all chars are 0-9, a-f, A-F)
            let is_hex = s.len() % 2 == 0 && 
                         s.len() > 0 &&
                         s.chars().all(|c| c.is_ascii_hexdigit());
            
            if is_hex {
                // Decode from hex
                match hex_to_bytes_into_slice(&s, buffer) {
                    Ok(len) => return Ok(len),
                    Err(_) => {
                        // If hex decoding fails, fall through to UTF-8 copy
                    }
                }
            }
            
            // Copy as UTF-8 bytes
            let bytes = s.as_bytes();
            let copy_len = core::cmp::min(bytes.len(), buffer.len());
            buffer[..copy_len].copy_from_slice(&bytes[..copy_len]);
            return Ok(copy_len);
        }

        // if array of numbers
        if item.is_array() {
            let size = item.get_array_size()?;
            let copy_len = core::cmp::min(size, buffer.len());
            for i in 0..copy_len {
                let elem = item.get_array_item(i)?;
                let val = elem.get_int_value()? as i32;
                buffer[i] = val as u8;
            }
            return Ok(copy_len);
        }

        Err(CJsonError::TypeError)
    }

    fn deserialize_string(&mut self, name: &str) -> core::result::Result<String, Self::Error> {
        let item = self.get_item(name)?;
        if item.is_string() {
            item.get_string_value()
        } else if item.is_number() {
            let n = item.get_number_value()?;
            let mut s = String::new();
            let _ = write!(&mut s, "{}", n);
            Ok(s)
        } else {
            Err(CJsonError::TypeError)
        }
    }

    fn deserialize_vec<T>(&mut self, name: &str) -> core::result::Result<Vec<T>, Self::Error>
    where
        T: Deserialize {
        let item = self.get_item(name)?;
        if !item.is_array() {
            return Err(CJsonError::TypeError);
        }

        let size = item.get_array_size()?;
        let mut out: Vec<T> = Vec::new();

        for i in 0..size {
            let elem_ref = item.get_array_item(i)?;
            // duplicate element and push as current context
            let dup_ptr = unsafe { cJSON_Duplicate(elem_ref.as_ptr(), 1) };
            let obj = unsafe { CJson::from_ptr(dup_ptr) }?;
            let mut idx_s = String::new();
            let _ = write!(&mut idx_s, "{}", i);
            let key = [name, "[", idx_s.as_str(), "]"].concat();
            self.stack_name.push(key.clone());
            self.stack.insert(key.clone(), obj);

            // let the element's Deserialize implementation operate on current top (use empty name)
            let v = T::deserialize(self, "")?;
            out.push(v);

            // pop element context
            let last = self.stack_name.pop().unwrap();
            let _ = self.stack.remove(&last);
        }

        Ok(out)
    }

    fn deserialize_array<T, const N: usize>(&mut self, name: &str) -> core::result::Result<[T; N], Self::Error>
    where
        T: Deserialize {
        let vec: Vec<T> = self.deserialize_vec(name)?;
        if vec.len() != N {
            return Err(CJsonError::InvalidOperation);
        }

        // convert Vec<T> into [T; N]
        let mut arr: core::mem::MaybeUninit<[T; N]> = core::mem::MaybeUninit::uninit();
        let ptr = arr.as_mut_ptr() as *mut T;
        for (i, v) in vec.into_iter().enumerate() {
            unsafe { ptr.add(i).write(v); }
        }
        Ok(unsafe { arr.assume_init() })
    }

    /// Begin deserializing a struct with the given name.
    fn deserialize_struct_start(&mut self, name: &str) -> core::result::Result<(), Self::Error> {
        // If name is empty, the caller intends to use the current top of stack
        if name == "" {
            return Ok(());
        }

        // get current container
        let cur_key = match self.stack_name.last() {
            Some(k) => k.clone(),
            None => return Err(CJsonError::InvalidOperation),
        };

        let container = match self.stack.get(&cur_key) {
            Some(c) => c,
            None => return Err(CJsonError::InvalidOperation),
        };

        // find the named field and duplicate it to own a copy for nested deserialization
        let item_ref = container.get_object_item(name)?;
        let dup_ptr = unsafe { cJSON_Duplicate(item_ref.as_ptr(), 1) };
        let obj = unsafe { CJson::from_ptr(dup_ptr) }?;

        self.stack_name.push(String::from(name));
        self.stack.insert(String::from(name), obj);

        Ok(())
    }

    /// Deserialize a struct field with name.
    fn deserialize_field<T>(&mut self, name: &str) -> core::result::Result<T, Self::Error>
    where
        T: Deserialize
    {

        T::deserialize(self, name)
    }

    /// End deserializing a struct.
    fn deserialize_struct_end(&mut self) -> core::result::Result<(), Self::Error> {
        // pop current nested object unless we're at root
        if self.stack_name.len() > 1 {
            if let Some(name) = self.stack_name.pop() {
                let _ = self.stack.remove(&name);
            }
        }

        Ok(())
    }


}

impl JsonDeserializer {
    fn get_item(&mut self, name: &str) -> core::result::Result<CJsonRef, CJsonError> {
        // current top key
        let cur_key = match self.stack_name.last() {
            Some(k) => k.clone(),
            None => return Err(CJsonError::InvalidOperation),
        };

        let container = match self.stack.get(&cur_key) {
            Some(c) => c,
            None => return Err(CJsonError::InvalidOperation),
        };

        if name == "" {
            // return a reference to the current item itself
            let ptr = container.as_ptr() as *mut _;
            unsafe { CJsonRef::from_ptr(ptr) }
        } else {
            container.get_object_item(name)
        }
    }
}

impl JsonDeserializer {
    
    pub fn parse(json: &str) -> CJsonResult<Self>  {


        let mut stack = BTreeMap::<String, CJson>::new();
        stack.insert(String::from(""), CJson::parse(json)?);

        Ok(Self {
            stack,
            stack_name: vec![String::from("")],
        })
    }

    pub fn drop(&mut self) {
        if let Some(obj) = self.stack.first_entry() {
            obj.get().drop();
        }
        self.stack.clear();
        self.stack_name.clear();
    }

}