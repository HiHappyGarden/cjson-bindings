/***************************************************************************
 *
 * cJSON FFI BINDING FOR RUST - Test Array Deserialization Bug Fix
 * Copyright (C) 2026 Antonio Salsi <passy.linux@zresa.it>
 *
 * This test verifies the fix for array deserialization issue where
 * struct_end was incorrectly popping from the stack for array elements.
 *
 ***************************************************************************/

use cjson_binding::{to_json, from_json};
use osal_rs_serde::{Serialize, Deserialize};
use osal_rs::utils::Bytes;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct UserConfig {
    user: Bytes<32>,
    password: Bytes<64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct WifiConfig {
    ssid: Bytes<32>,
    password: Bytes<64>,
    hostname: Bytes<32>,
    auth: u8,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct NtpConfig {
    server: Bytes<32>,
    port: u16,
    msg_len: u16,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Config {
    version: u8,
    serial: Bytes<16>,
    timezone: u16,
    daylight_saving_time: bool,
    users: [UserConfig; 2],
    wifi: WifiConfig,
    ntp: NtpConfig,
}

#[test]
fn test_config_deserialization_from_json() {
    // This is the exact JSON that was failing before the fix
    let json = r#"{
        "version":0,
        "serial":"",
        "timezone":60,
        "daylight_saving_time":true,
        "users":[
            {"user":"","password":""},
            {"user":"","password":""}
        ],
        "wifi":{
            "ssid":"",
            "password":"",
            "hostname":"hi-happy-garden-rs",
            "auth":3,
            "enabled":false
        },
        "ntp":{
            "server":"",
            "port":123,
            "msg_len":48,
            "enabled":false
        }
    }"#;

    // This should NOT fail with "Invalid operation" anymore
    let config: Config = from_json(&json.to_string())
        .expect("Failed to deserialize config - the bug is not fixed!");

    // Verify the deserialized values
    assert_eq!(config.version, 0);
    assert_eq!(config.timezone, 60);
    assert_eq!(config.daylight_saving_time, true);
    assert_eq!(config.users.len(), 2);
    assert_eq!(config.wifi.auth, 3);
    assert_eq!(config.wifi.enabled, false);
    assert_eq!(config.ntp.port, 123);
    assert_eq!(config.ntp.msg_len, 48);
    assert_eq!(config.ntp.enabled, false);
    
    // Verify hostname conversion
    let hostname_str = config.wifi.hostname.as_str();
    assert_eq!(hostname_str, "hi-happy-garden-rs");
}

#[test]
fn test_roundtrip_serialization_with_arrays() {
    let original = Config {
        version: 1,
        serial: Bytes::new_by_str("ABC123"),
        timezone: 120,
        daylight_saving_time: false,
        users: [
            UserConfig {
                user: Bytes::new_by_str("admin"),
                password: Bytes::new_by_str("pass1"),
            },
            UserConfig {
                user: Bytes::new_by_str("guest"),
                password: Bytes::new_by_str("pass2"),
            },
        ],
        wifi: WifiConfig {
            ssid: Bytes::new_by_str("MyNetwork"),
            password: Bytes::new_by_str("secret"),
            hostname: Bytes::new_by_str("device"),
            auth: 3,
            enabled: true,
        },
        ntp: NtpConfig {
            server: Bytes::new_by_str("pool.ntp.org"),
            port: 123,
            msg_len: 48,
            enabled: true,
        },
    };

    // Serialize to JSON
    let json = to_json(&original).expect("Failed to serialize");
    println!("Serialized JSON:\n{}", json);

    // Verify it's valid JSON with array notation
    assert!(json.contains(r#""users":["#), "Users should be an array");
    assert!(json.contains(r#""wifi":{"#), "Wifi should be an object");
    assert!(json.contains(r#""ntp":{"#), "NTP should be an object");

    // Deserialize back
    let deserialized: Config = from_json(&json).expect("Failed to deserialize");

    // Verify all fields match
    assert_eq!(deserialized.version, original.version);
    assert_eq!(deserialized.serial.as_str(), original.serial.as_str());
    assert_eq!(deserialized.timezone, original.timezone);
    assert_eq!(deserialized.daylight_saving_time, original.daylight_saving_time);
    
    // Verify array elements
    assert_eq!(deserialized.users[0].user.as_str(), "admin");
    assert_eq!(deserialized.users[0].password.as_str(), "pass1");
    assert_eq!(deserialized.users[1].user.as_str(), "guest");
    assert_eq!(deserialized.users[1].password.as_str(), "pass2");
    
    // Verify nested structs
    assert_eq!(deserialized.wifi.ssid.as_str(), "MyNetwork");
    assert_eq!(deserialized.wifi.enabled, true);
    assert_eq!(deserialized.ntp.server.as_str(), "pool.ntp.org");
    assert_eq!(deserialized.ntp.enabled, true);
}
