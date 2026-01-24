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

use crate::CJsonResult;
use crate::cjson::CJsonError;
use crate::cjson::CJson;

use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;


pub struct JsonDeserializer<T> 
where 
    T: Deserialize + Default
{
    stack: BTreeMap<String, CJson>,
    stack_name: Vec<String>,
    t: T
}

impl<T> Deserializer for JsonDeserializer<T> 
where
    T: Deserialize + Default
{
    type Error = CJsonError;
    

    /// Begin deserializing a struct with the given name.
    fn deserialize_struct_start(&mut self, name: &str) -> core::result::Result<(), Self::Error> {

        if name != "" {
            // self.stack.push(self.obj.clone());
            self.stack_name.push(String::from(""));
            self.stack.insert(String::from(""), CJson::create_object()?);

            Ok(())
        } else {

            let len = self.stack.len();
            if len < 1 {
                return Err(CJsonError::InvalidOperation);    
            }
            let len = len - 1;


            let key  = &self.stack_name[len];
            if let Some(phader_obj) = self.stack.get_mut(key) {

                // self.stack_name.push(String::from(name));
                // self.stack.insert(String::from(name), obj);
                Ok(())
            } else {
                Err(CJsonError::InvalidOperation)
            }
        }
    }

    /// Deserialize a struct field with name.
    fn deserialize_field<Y>(&mut self, name: &str) -> core::result::Result<Y, Self::Error>
    where
        Y: Deserialize
    {

        Y::deserialize(self, name)
    }

    /// End deserializing a struct.
    fn deserialize_struct_end(&mut self) -> core::result::Result<(), Self::Error> {
        Ok(())
    }


    // fn serialize_struct_start(&mut self, name: &str, _len: usize) -> Result<(), Self::Error> {

    //     if name == "" {
    //         // self.stack.push(self.obj.clone());
    //         self.stack_name.push(String::from(""));
    //         self.stack.insert(String::from(""), CJson::create_object()?);

    //         Ok(())
    //     } else {

    //         let len = self.stack.len();
    //         if len < 1 {
    //             return Err(CJsonError::InvalidOperation);    
    //         }
    //         let len = len - 1;


    //         let key  = &self.stack_name[len];
    //         if let Some(phader_obj) = self.stack.get_mut(key) {

    //             let obj = CJson::create_object()?;
    //             phader_obj.add_item_to_object(name, obj.clone())?;
    //             self.stack_name.push(String::from(name));
    //             self.stack.insert(String::from(name), obj);
    //             Ok(())
    //         } else {
    //             Err(CJsonError::InvalidOperation)
    //         }
            
        

    //     }


    // }


    // fn serialize_struct_end(&mut self) -> Result<(), Self::Error> {
        
    //     self.stack_name.pop();

    //     Ok(())
    // }

    fn deserialize_bool(&mut self, name: &str) -> core::result::Result<bool, Self::Error> {
        todo!()
    }
    
    fn deserialize_u8(&mut self, name: &str) -> core::result::Result<u8, Self::Error> {
        todo!()
    }
    
    fn deserialize_i8(&mut self, name: &str) -> core::result::Result<i8, Self::Error> {
        todo!()
    }
    
    fn deserialize_u16(&mut self, name: &str) -> core::result::Result<u16, Self::Error> {
        todo!()
    }
    
    fn deserialize_i16(&mut self, name: &str) -> core::result::Result<i16, Self::Error> {
        todo!()
    }
    
    fn deserialize_u32(&mut self, name: &str) -> core::result::Result<u32, Self::Error> {
        todo!()
    }
    
    fn deserialize_i32(&mut self, name: &str) -> core::result::Result<i32, Self::Error> {
        todo!()
    }
    
    fn deserialize_u64(&mut self, name: &str) -> core::result::Result<u64, Self::Error> {
        todo!()
    }
    
    fn deserialize_i64(&mut self, name: &str) -> core::result::Result<i64, Self::Error> {
        todo!()
    }
    
    fn deserialize_u128(&mut self, name: &str) -> core::result::Result<u128, Self::Error> {
        todo!()
    }
    
    fn deserialize_i128(&mut self, name: &str) -> core::result::Result<i128, Self::Error> {
        todo!()
    }
    
    fn deserialize_f32(&mut self, name: &str) -> core::result::Result<f32, Self::Error> {
        todo!()
    }
    
    fn deserialize_f64(&mut self, name: &str) -> core::result::Result<f64, Self::Error> {
        todo!()
    }
    
    fn deserialize_bytes(&mut self, name: &str, buffer: &mut [u8]) -> core::result::Result<usize, Self::Error> {
        todo!()
    }
    
    fn deserialize_string(&mut self, name: &str) -> core::result::Result<String, Self::Error> {
        todo!()
    }
    
    fn deserialize_vec<Y>(&mut self, name: &str) -> core::result::Result<Vec<Y>, Self::Error> 
    where 
        Y: Deserialize {
        todo!()
    }
    
    fn deserialize_array<Y, const N: usize>(&mut self, name: &str) -> core::result::Result<[Y; N], Self::Error> 
    where 
        Y: Deserialize {
        todo!()
    }


}

impl<T> JsonDeserializer<T> 
where 
    T: Deserialize + Default
{
    
    pub fn parse(json: &str) -> CJsonResult<Self>  {


        let mut stack = BTreeMap::<String, CJson>::new();
        stack.insert(String::from(""), CJson::parse(json)?);

        Ok(Self {
            stack,
            stack_name: vec![String::from("")],
            t: T::default(),
        })
    }

}