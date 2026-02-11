/***************************************************************************
 *
 * cJSON FFI BINDING FOR RUST - Array Serialization Example
 * Copyright (C) 2026 Antonio Salsi <passy.linux@zresa.it>
 *
 * This example demonstrates the fix for array serialization bug.
 * Arrays are now correctly serialized as JSON arrays instead of
 * duplicate object keys.
 *
 ***************************************************************************/

extern crate alloc;

use alloc::format;
use cjson_binding::{to_json, from_json};
use osal_rs_serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserConfig {
    user: u32,
    password: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    version: u8,
    timezone: u16,
    daylight_saving_time: bool,
    users: [UserConfig; 2],
}

fn main() {
    println!("=== Array Serialization Fix Example ===\n");
    
    let config = Config {
        version: 0,
        timezone: 60,
        daylight_saving_time: true,
        users: [
            UserConfig { user: 1001, password: 2002 },
            UserConfig { user: 3003, password: 4004 },
        ],
    };

    println!("Original Config:");
    println!("  Version: {}", config.version);
    println!("  Timezone: {}", config.timezone);
    println!("  Daylight Saving: {}", config.daylight_saving_time);
    println!("  Users[0]: user={}, password={}", config.users[0].user, config.users[0].password);
    println!("  Users[1]: user={}, password={}", config.users[1].user, config.users[1].password);
    
    // Serialize to JSON
    let json_str = match to_json(&config) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Serialization error: {:?}", e);
            return;
        }
    };
    
    println!("\n--- Serialized JSON (Fixed!) ---");
    println!("{}", json_str);
    println!("--------------------------------\n");
    
    // Check that it's an array (not duplicate keys)
    let users_count = json_str.matches(r#""users""#).count();
    println!("Verification:");
    println!("  'users' appears {} time(s) as a key (should be 1)", users_count);
    
    if json_str.contains("[") && json_str.contains("]") {
        println!("  ✓ JSON contains array notation [ ]");
    } else {
        println!("  ✗ JSON missing array notation!");
    }
    
    // Deserialize back
    println!("\nDeserializing...");
    match from_json::<Config>(&json_str) {
        Ok(deserialized) => {
            println!("✓ Deserialization successful!");
            println!("  Version: {}", deserialized.version);
            println!("  Users[0]: user={}, password={}", deserialized.users[0].user, deserialized.users[0].password);
            println!("  Users[1]: user={}, password={}", deserialized.users[1].user, deserialized.users[1].password);
        }
        Err(e) => {
            eprintln!("✗ Deserialization failed: {:?}", e);
        }
    }
    
    println!("\n=== Example Complete ===");
}
