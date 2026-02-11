/***************************************************************************
 *
 * cJSON FFI BINDING FOR RUST - Test for Array Serialization
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

use cjson_binding::{to_json, from_json};
use osal_rs_serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct UserConfig {
    user: u32,
    password: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
struct Config {
    version: u8,
    users: [UserConfig; 2],
}

#[test]
fn test_array_serialization() {
    let config = Config {
        version: 1,
        users: [
            UserConfig { user: 100, password: 200 },
            UserConfig { user: 300, password: 400 },
        ],
    };

    // Serialize to JSON
    let json_str = to_json(&config).expect("Failed to serialize");
    
    println!("Serialized JSON: {}", json_str);
    
    // Verify that JSON contains an array (not duplicate keys)
    assert!(json_str.contains(r#""users":"#), "JSON should contain 'users' field");
    assert!(json_str.contains(r#"["#), "JSON should contain array notation");
    
    // Count occurrences of "users" - should appear only once as a key
    let users_count = json_str.matches(r#""users""#).count();
    assert_eq!(users_count, 1, "JSON should contain 'users' key exactly once, not duplicated");
    
    // Deserialize back
    let deserialized: Config = from_json(&json_str).expect("Failed to deserialize");
    
    // Verify the data matches
    assert_eq!(deserialized.version, config.version);
    assert_eq!(deserialized.users, config.users);
    assert_eq!(deserialized.users[0].user, 100);
    assert_eq!(deserialized.users[0].password, 200);
    assert_eq!(deserialized.users[1].user, 300);
    assert_eq!(deserialized.users[1].password, 400);
}

#[test]
fn test_empty_array_serialization() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct EmptyArrayConfig {
        items: [u32; 0],
    }

    let config = EmptyArrayConfig {
        items: [],
    };

    let json_str = to_json(&config).expect("Failed to serialize");
    println!("Empty array JSON: {}", json_str);
    
    let deserialized: EmptyArrayConfig = from_json(&json_str).expect("Failed to deserialize");
    assert_eq!(deserialized, config);
}
