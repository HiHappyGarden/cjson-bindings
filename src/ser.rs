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

use core::result::Result;

use osal_rs_serde::{Serialize, Serializer};

use crate::CJsonResult;
use crate::cjson::CJsonError;
use crate::cjson::CJson;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;


pub struct JsonSerializer {
    stack: BTreeMap<String, CJson>,
    stack_name: Vec<String>,
}


impl Serializer for JsonSerializer {
    type Error =  CJsonError;

    fn serialize_bool(&mut self, name: &str, v: bool) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_bool(v)?)?;
        } else {
            container.add_bool_to_object(name, v)?;
        }
        Ok(())
    }


    fn serialize_u8(&mut self, name: &str, v: u8) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_i8(&mut self, name: &str, v: i8) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_u16(&mut self, name: &str, v: u16) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_i16(&mut self, name: &str, v: i16) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_u32(&mut self, name: &str, v: u32) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_i32(&mut self, name: &str, v: i32) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_u64(&mut self, name: &str, v: u64) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_i64(&mut self, name: &str, v: i64) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_u128(&mut self, name: &str, v: u128) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_i128(&mut self, name: &str, v: i128) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_f32(&mut self, name: &str, v: f32) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v as f64)?)?;
        } else {
            container.add_number_to_object(name, v as f64)?;
        }
        Ok(())
    }

    fn serialize_f64(&mut self, name: &str, v: f64) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_number(v)?)?;
        } else {
            container.add_number_to_object(name, v)?;
        }
        Ok(())
    }

    fn serialize_bytes(&mut self, name: &str, v: &[u8]) -> Result<(), Self::Error> {
        // Create a string for hex encoding
        let mut hex_string = String::new();
        for &byte in v {
            use core::fmt::Write;
            let _ = write!(&mut hex_string, "{:02x}", byte);
        }

        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_string(&hex_string)?)?;
        } else {
            container.add_string_to_object(name, &hex_string)?;
        }
        Ok(())
    }

    fn serialize_string(&mut self, name: &str, v: &String) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_string(v)?)?;
        } else {
            container.add_string_to_object(name, v)?;
        }
        Ok(())
    }

    fn serialize_str(&mut self, name: &str, v: &str) -> Result<(), Self::Error> {
        let container = self.get_current_object()?;
        if container.is_array() {
            container.add_item_to_array(CJson::create_string(v)?)?;
        } else {
            container.add_string_to_object(name, v)?;
        }
        Ok(())
    }

    fn serialize_vec<T>(&mut self, name: &str, v: &Vec<T>) -> Result<(), Self::Error>
    where
        T: Serialize {
        // Create a JSON array
        let array = CJson::create_array()?;
        
        // Add the array to the parent object
        self.get_current_object()?.add_item_to_object(name, array.clone())?;
        
        // Push array onto stack
        self.stack_name.push(String::from(name));
        self.stack.insert(String::from(name), array);
        
        // Serialize each item into the array
        for item in v.iter() {
            // Serialize the item with empty name (will be added to array, not as named field)
            item.serialize("", self)?;
        }
        
        // Pop array from stack
        self.stack_name.pop();
        
        Ok(())
    }

    fn serialize_array<T>(&mut self, name: &str, v: &[T]) -> Result<(), Self::Error>
    where
        T: Serialize {
        // Create a JSON array
        let array = CJson::create_array()?;
        
        // Add the array to the parent object
        self.get_current_object()?.add_item_to_object(name, array.clone())?;
        
        // Push array onto stack
        self.stack_name.push(String::from(name));
        self.stack.insert(String::from(name), array);
        
        // Serialize each item into the array
        for item in v.iter() {
            // Serialize the item with empty name (will be added to array, not as named field)
            item.serialize("", self)?;
        }
        
        // Pop array from stack
        self.stack_name.pop();
        
        Ok(())
    }

    fn serialize_struct_start(&mut self, name: &str, _len: usize) -> Result<(), Self::Error> {

        if name == "" {
            // Check if we're in an array (for array of structs) or creating the root object
            if let Some(last_name) = self.stack_name.last() {
                if let Some(container) = self.stack.get_mut(last_name) {
                    if container.is_array() {
                        // We're serializing a struct that will be added to an array
                        let obj = CJson::create_object()?;
                        container.add_item_to_array(obj.clone())?;
                        
                        // Use a unique key for this array element
                        let unique_key = format!("{}[{}]", last_name, container.get_array_size()? - 1);
                        self.stack_name.push(unique_key.clone());
                        self.stack.insert(unique_key, obj);
                        return Ok(());
                    }
                }
            }
            
            // Root object case
            self.stack_name.push(String::from(""));
            self.stack.insert(String::from(""), CJson::create_object()?);

            Ok(())
        } else {

            let len = self.stack_name.len();
            if len < 1 {
                return Err(CJsonError::InvalidOperation);
            }
            let len = len - 1;


            let key  = &self.stack_name[len];
            if let Some(phader_obj) = self.stack.get_mut(key) {

                let obj = CJson::create_object()?;
                phader_obj.add_item_to_object(name, obj.clone())?;
                self.stack_name.push(String::from(name));
                self.stack.insert(String::from(name), obj);
                Ok(())
            } else {
                Err(CJsonError::InvalidOperation)
            }
        }
    }

    fn serialize_struct_end(&mut self) -> Result<(), Self::Error> {

        self.stack_name.pop();

        Ok(())
    }
    
    

}


impl JsonSerializer {

    pub fn new() -> Self {

        Self {
            stack: BTreeMap::new(),
            stack_name: Vec::new(),
        }
    }

    pub fn print(&mut self) -> CJsonResult<String> {

        if let Some(obj) = self.stack.first_entry() {
            let obj = obj.get();
            let ret = obj.print();
            obj.drop();
            ret
        } else {
            Err(CJsonError::NotFound)
        }

    }

    pub fn print_unformatted(&mut self) -> CJsonResult<String> {
        if let Some(obj) = self.stack.first_entry() {
            let obj = obj.get();
            let ret = obj.print_unformatted();
            obj.drop();
            ret
        } else {
            Err(CJsonError::NotFound)
        }
    }

    fn get_current_object(&mut self) -> CJsonResult<&mut CJson> {
        if let Some(name) = self.stack_name.last() {
            if let Some(obj) = self.stack.get_mut(name) {
                return Ok(obj);
            }
        }
        

        Err(CJsonError::InvalidOperation)
    }
} 