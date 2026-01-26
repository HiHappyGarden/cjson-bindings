# cjson-binding

Safe Rust bindings for the [cJSON](https://github.com/DaveGamble/cJSON) library - a lightweight JSON parser in C.

## Overview

`cjson-binding` provides idiomatic, safe Rust wrappers around the cJSON C library, offering:

- **Safe API**: Memory-safe wrappers with automatic resource management (RAII)
- **Type-safe operations**: Strong typing with `Result` types for error handling
- **JSON Pointer support (RFC6901)**: Navigate JSON documents using JSON Pointer syntax
- **JSON Patch support (RFC6902)**: Generate and apply JSON patches
- **JSON Merge Patch support (RFC7386)**: Generate and apply merge patches
- **Serialization/Deserialization**: Full support for `#[derive(Serialize, Deserialize)]` with osal-rs-serde integration
- **no_std compatible**: Suitable for embedded systems with built-in allocator and panic handler

## Embedded & no_std Support

The library is designed for embedded systems and supports `no_std` environments:

- **Global Allocator**: Uses C's `malloc`/`free` via FFI
- **Default Panic Handler**: Provides a simple infinite loop panic handler
- **Custom Panic Handler**: Use the `disable_panic` feature to provide your own

### Cargo Features

- **`std`**: Enables standard library support (required for tests)
- **`disable_panic`**: Disables both the default allocator and panic handler, allowing you to provide your own
- **`osal_rs`**: Enables integration with osal-rs and osal-rs-serde for automatic serialization/deserialization with `#[derive]` macros

**Example with custom allocator and panic handler:**
```toml
[dependencies]
cjson-binding = { version = "0.6.0", features = ["disable_panic"] }
```

Then provide your own in your application:
```rust
use core::alloc::{GlobalAlloc, Layout};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Your custom allocation
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Your custom deallocation
    }
}

#[global_allocator]
static ALLOCATOR: MyAllocator = MyAllocator;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    // Your custom panic handling
    loop {}
}
```

## Features

### Core JSON Operations

- Parse and print JSON with automatic memory management
- Create and manipulate JSON objects, arrays, strings, numbers, booleans, and null values
- Type checking and value retrieval with compile-time safety
- Deep cloning and comparison

### Advanced Features

- **JSON Pointer (RFC6901)**: Navigate JSON structures using paths like `/foo/bar/0`
- **JSON Patch (RFC6902)**: Generate and apply patch operations (add, remove, replace, move, copy, test)
- **JSON Merge Patch (RFC7386)**: Simpler patch format for common use cases
- Sorting object keys alphabetically

### Serialization & Deserialization (with `osal_rs` feature)

When the `osal_rs` feature is enabled, `cjson-binding` provides full integration with the `osal-rs-serde` framework for automatic serialization and deserialization:

- **Derive Macros**: Use `#[derive(Serialize, Deserialize)]` on your structs
- **Type-safe**: Compile-time guarantees for data conversion
- **Memory-efficient**: Direct JSON creation/parsing without intermediate allocations
- **Nested structures**: Full support for complex nested data structures
- **Easy API**: Simple `to_json()` and `from_json()` functions

#### Supported Types

##### Primitives
- **Integers**: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`
- **Floats**: `f32`, `f64`
- **Boolean**: `bool`

##### Compound Types
- **Arrays**: `[T; N]` for any serializable type T
- **Vec**: `Vec<T>` for dynamic arrays
- **String**: `String` and `&str`
- **Bytes**: `&[u8]` (serialized as hexadecimal string)

##### Custom Types
- Any struct with `#[derive(Serialize, Deserialize)]`
- Nested struct composition fully supported

##### JSON Format & Type Mapping

The JSON serializer/deserializer maps Rust types to JSON as follows:

```
bool       → JSON boolean (true/false)
integers   → JSON number (converted to f64)
floats     → JSON number
String/str → JSON string
&[u8]      → JSON string (hexadecimal representation)
Vec<T>     → JSON array
[T; N]     → JSON array
struct     → JSON object
```

**Note**: All integer types (u8-u128, i8-i128) are converted to/from JSON numbers (f64). Be aware of potential precision loss for values larger than 2^53 (JavaScript number limitations).

## Installation

Add to your `Cargo.toml`:

### Basic Usage (JSON parsing only)

```toml
[dependencies]
cjson-binding = "0.6.0"
```

### With Serialization Support

For automatic serialization/deserialization with derive macros:

```toml
[dependencies]
cjson-binding = { version = "0.6.0", features = ["osal_rs"] }
```

### For Embedded Systems (no_std)

```toml
[dependencies]
cjson-binding = { version = "0.6.0", default-features = false }
```

### With Custom Allocator and Panic Handler

```toml
[dependencies]
cjson-binding = { version = "0.6.0", features = ["disable_panic"] }
```

Available features:
- `std`: Enables standard library support (default: disabled)
- `disable_panic`: Disables default allocator and panic handler (default: disabled)
- `osal_rs`: Enables osal-rs-serde integration for serialization (default: disabled)

## Usage

### Basic Example

```rust
use cjson_rs::{CJson, CJsonResult};

fn main() -> CJsonResult<()> {
    // Parse JSON
    let json = CJson::parse(r#"{"name": "John", "age": 30}"#)?;
    
    // Access values
    let name = json.get_object_item("name")?;
    println!("Name: {}", name.get_string_value()?);
    
    // Create new JSON
    let mut obj = CJson::create_object()?;
    obj.add_string_to_object("city", "New York")?;
    obj.add_number_to_object("population", 8_000_000.0)?;
    
    // Print JSON
    println!("{}", obj.print()?);
    
    Ok(())
}
```

### Serialization & Deserialization Examples (with `osal_rs` feature)

#### Basic Struct Serialization

```rust
use osal_rs_serde::{Serialize, Deserialize};
use cjson_rs::{to_json, from_json};

#[derive(Serialize, Deserialize, Default)]
struct SensorData {
    temperature: i16,
    humidity: u8,
    pressure: u32,
}

fn main() {
    // Create a structure
    let data = SensorData {
        temperature: 25,
        humidity: 60,
        pressure: 1013,
    };

    // Serialize to JSON string
    let json_str = to_json(&data).unwrap();
    println!("JSON: {}", json_str);
    // Output: {"temperature":25,"humidity":60,"pressure":1013}

    // Deserialize from JSON string
    let read_data: SensorData = from_json(&json_str).unwrap();
    println!("Temperature: {}", read_data.temperature);
}
```

#### Nested Structs

```rust
use osal_rs_serde::{Serialize, Deserialize};
use cjson_rs::{to_json, from_json};

#[derive(Serialize, Deserialize, Default)]
struct Location {
    latitude: i32,
    longitude: i32,
}

#[derive(Serialize, Deserialize, Default)]
struct Device {
    id: u32,
    battery: u8,
    location: Location,
    active: bool,
}

fn main() {
    let device = Device {
        id: 42,
        battery: 85,
        location: Location {
            latitude: 45500000,
            longitude: 9200000,
        },
        active: true,
    };

    // Serialize
    let json_str = to_json(&device).unwrap();
    println!("{}", json_str);
    // Output: {"id":42,"battery":85,"location":{"latitude":45500000,"longitude":9200000},"active":true}

    // Deserialize
    let decoded: Device = from_json(&json_str).unwrap();
    println!("Device at {}, {}", 
             decoded.location.latitude, 
             decoded.location.longitude);
}
```

#### Arrays and Vectors

```rust
use osal_rs_serde::{Serialize, Deserialize};
use cjson_rs::{to_json, from_json};
use alloc::vec::Vec;

#[derive(Serialize, Deserialize, Default)]
struct TelemetryPacket {
    timestamp: u64,
    samples: [u16; 4],      // Fixed-size array
    tags: Vec<u8>,          // Dynamic vector
}

fn main() {
    let packet = TelemetryPacket {
        timestamp: 1642857600,
        samples: [10, 20, 30, 40],
        tags: vec![1, 2, 3],
    };

    let json_str = to_json(&packet).unwrap();
    println!("{}", json_str);
    // Output: {"timestamp":1642857600,"samples":[10,20,30,40],"tags":[1,2,3]}

    let decoded: TelemetryPacket = from_json(&json_str).unwrap();
}
```

#### Complex Embedded System Example

```rust
use osal_rs_serde::{Serialize, Deserialize};
use cjson_rs::{to_json, from_json};

#[derive(Serialize, Deserialize, Default)]
struct MotorControl {
    motor_id: u8,
    speed: i16,        // -1000 to 1000
    direction: bool,   // true = forward, false = reverse
    current: u16,      // mA
}

#[derive(Serialize, Deserialize, Default)]
struct RobotState {
    timestamp: u64,
    motors: [MotorControl; 4],  // 4 motors
    battery_voltage: u16,        // mV
    temperature: i8,             // °C
    error_flags: u32,
}

fn main() {
    let state = RobotState {
        timestamp: 1000000,
        motors: [
            MotorControl { motor_id: 0, speed: 500, direction: true, current: 1200 },
            MotorControl { motor_id: 1, speed: 500, direction: true, current: 1150 },
            MotorControl { motor_id: 2, speed: -300, direction: false, current: 800 },
            MotorControl { motor_id: 3, speed: -300, direction: false, current: 850 },
        ],
        battery_voltage: 12400,  // 12.4V
        temperature: 35,
        error_flags: 0,
    };

    // Serialize to JSON
    let json_str = to_json(&state).unwrap();
    println!("Robot state: {}", json_str);
    
    // Deserialize back
    let decoded: RobotState = from_json(&json_str).unwrap();
    println!("Battery: {}mV, Temp: {}°C", 
             decoded.battery_voltage, 
             decoded.temperature);
}
```

### JSON Pointer Example

```rust
use cjson_rs::{CJson, JsonPointer};

let json = CJson::parse(r#"{
    "users": [
        {"name": "Alice", "age": 25},
        {"name": "Bob", "age": 30}
    ]
}"#)?;

// Navigate using JSON Pointer
let bob = JsonPointer::get(&json, "/users/1/name")?;
println!("User: {}", bob.get_string_value()?); // "Bob"
```

### JSON Patch Example

```rust
use cjson_rs::{CJson, JsonPatch};

let mut original = CJson::parse(r#"{"name": "John", "age": 30}"#)?;
let patches = CJson::parse(r#"[
    {"op": "replace", "path": "/age", "value": 31},
    {"op": "add", "path": "/city", "value": "NYC"}
]"#)?;

// Apply patches
JsonPatch::apply(&mut original, &patches)?;
println!("{}", original.print()?);
// Output: {"name":"John","age":31,"city":"NYC"}
```

### JSON Merge Patch Example

```rust
use cjson_rs::{CJson, JsonMergePatch};

let mut target = CJson::parse(r#"{"name": "John", "age": 30}"#)?;
let patch = CJson::parse(r#"{"age": 31, "city": "NYC"}"#)?;

// Apply merge patch
let result = JsonMergePatch::apply(&mut target, &patch)?;
println!("{}", result.print()?);
```

## API Types

### Main Types

- **`CJson`**: Owned JSON value with automatic memory management
- **`CJsonRef`**: Borrowed reference to a JSON value (non-owning)
- **`CJsonResult<T>`**: Result type for operations that can fail
- **`CJsonError`**: Error enumeration for all possible errors

### Utility Types

- **`JsonPointer`**: JSON Pointer (RFC6901) operations
- **`JsonPatch`**: JSON Patch (RFC6902) operations
- **`JsonMergePatch`**: JSON Merge Patch (RFC7386) operations
- **`JsonUtils`**: Additional utilities (e.g., sorting)

### Serialization Types (with `osal_rs` feature)

- **`JsonSerializer`**: Serializes Rust types to JSON format
- **`JsonDeserializer`**: Deserializes JSON to Rust types
- **`to_json<T>(&T) -> Result<String>`**: High-level serialization function
- **`from_json<T>(&String) -> Result<T>`**: High-level deserialization function

## Error Handling

All operations that can fail return `CJsonResult<T>`, which is a type alias for `Result<T, CJsonError>`:

```rust
pub enum CJsonError {
    ParseError,
    NullPointer,
    InvalidUtf8,
    NotFound,
    TypeError,
    AllocationError,
    InvalidOperation,
}
```

## Memory Safety

`` ensures memory safety through:

- **RAII**: `CJson` automatically frees memory when dropped
- **No manual memory management**: All allocations/deallocations are handled automatically
- **Reference types**: `CJsonRef` provides safe borrowing without ownership transfer
- **Clear ownership**: `into_raw()` for explicit ownership transfer when needed

## Best Practices

### 1. Use Derive Macros for Serialization

Always prefer derive macros with the `osal_rs` feature for automatic serialization:

```rust
#[derive(Serialize, Deserialize, Default)]
struct MyStruct {
    // fields...
}
```

### 2. Handle Errors Appropriately

Always handle serialization/deserialization errors:

```rust
match to_json(&data) {
    Ok(json_str) => {
        // Use JSON string
        println!("Success: {}", json_str);
    }
    Err(e) => {
        // Handle error
        eprintln!("Serialization failed: {:?}", e);
    }
}
```

### 3. Use Type-Safe Access

Prefer type-safe methods over raw pointer manipulation:

```rust
// Good
let value = json.get_object_item("field")?.get_number_value()?;

// Avoid
let ptr = json.as_ptr(); // Manual pointer manipulation
```

### 4. Memory Management in Embedded Systems

For embedded systems, consider using custom allocator with `disable_panic` feature:

```rust
#[global_allocator]
static ALLOCATOR: MyAllocator = MyAllocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Your panic handling
    loop {}
}
```

### 5. Numeric Precision

Be aware of JSON number limitations (IEEE 754 double precision):

```rust
// Precise for values up to 2^53
let safe_value: u32 = 1_000_000;

// May lose precision
let large_value: u64 = 9_007_199_254_740_993; // > 2^53
```

### 6. Versioning for Compatibility

Add version fields for forward/backward compatibility:

```rust
#[derive(Serialize, Deserialize, Default)]
struct Message {
    version: u8,
    // other fields...
}
```

## Performance Considerations

### JSON Number Precision

All Rust integer types are converted to JSON numbers (IEEE 754 double precision float):

- **Safe range**: Values up to ±2^53 (9,007,199,254,740,992) maintain exact precision
- **Loss of precision**: Larger integers may lose precision during serialization/deserialization
- **Recommendation**: For values > 2^53, consider using strings instead

### Memory Usage

- **Stack allocation**: JSON structures are allocated on the heap via C's malloc
- **Automatic cleanup**: RAII ensures memory is freed when objects go out of scope
- **Zero-copy parsing**: String values reference the original JSON buffer when possible

### Embedded Systems Optimization

For resource-constrained environments:

```rust
// Pre-calculate expected JSON size
const EXPECTED_SIZE: usize = 256;

// Use stack buffers where possible
let json_str = to_json(&data).unwrap();
// Process immediately to free memory
process_json(&json_str);
```

## Comparison with Other Libraries

| Feature | cjson-binding | serde_json | json | tinyjson |
|---------|---------------|------------|------|----------|
| No-std support | ✅ Native | ✅ Via feature | ❌ | ✅ Native |
| Derive macros | ✅ (via osal_rs) | ✅ (via serde) | ❌ | ❌ |
| JSON Pointer (RFC6901) | ✅ Built-in | ✅ Via crate | ❌ | ❌ |
| JSON Patch (RFC6902) | ✅ Built-in | ✅ Via crate | ❌ | ❌ |
| JSON Merge Patch (RFC7386) | ✅ Built-in | ✅ Via crate | ❌ | ❌ |
| Binary size | **Small** | Medium | Small | **Very small** |
| Speed | **Fast** (C library) | Very fast | Medium | Medium |
| C library dependency | ✅ cJSON | ❌ | ❌ | ❌ |
| Memory allocator | C malloc | Rust | Rust | Rust |
| Embedded RTOS support | ✅ Excellent | ⚠️ Limited | ❌ | ⚠️ Limited |
| Learning curve | **Easy** | Moderate | Easy | Easy |

**Choose cjson-binding when:**
- Working in embedded/RTOS environments with C interoperability
- Need RFC6901/6902/7386 support out of the box
- Prefer battle-tested C library (cJSON is widely used)
- Want simple, straightforward API
- Integrating with existing C/C++ codebase

**Choose serde_json when:**
- Pure Rust solution preferred
- Maximum performance is critical
- Need extensive ecosystem integration
- Working primarily with std

**Choose tinyjson when:**
- Minimal binary size is top priority
- Don't need advanced features
- Simple JSON parsing only

## Dependencies

This crate links against the [cJSON](https://github.com/DaveGamble/cJSON) C library. You need to have cJSON installed or provide it as part of your build process.

## License

This Rust wrapper is licensed under the GNU General Public License v3.0 (GPL-3.0).

The underlying [cJSON library](https://github.com/DaveGamble/cJSON) is licensed under the MIT License.

### cJSON License

```
Copyright (c) 2009-2017 Dave Gamble and cJSON contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

See [LICENSE](LICENSE) for the full GPL-3.0 license text and [cJSON's license](https://github.com/DaveGamble/cJSON/blob/master/LICENSE) for details.

## Testing

The library includes comprehensive unit tests covering all major functionality. To run the tests:

```bash
cargo test --features std
```

Or use the provided alias:

```bash
cargo test-std
```

See [TESTS.md](TESTS.md) for detailed test documentation and coverage information.

### Test Requirements

- cJSON library installed on your system (`libcjson` and `libcjson_utils`)
- Standard library support (enabled via the `std` feature for testing)

### Running tests locally (linking cJSON built from source)

If you need to run the crate tests locally and link against a locally-built cJSON (useful for `no_std` embedded workflows where the project CMake already builds cJSON), follow these steps:

1. Clone and build cJSON for the host:

```bash
git clone --depth 1 --branch v1.7.19 https://github.com/DaveGamble/cJSON.git /path/to/build-host/cJSON
cmake -S /path/to/build-host/cJSON -B /path/to/build-host/cJSON/build -DBUILD_SHARED_AND_STATIC_LIBS=OFF -DENABLE_CJSON_UTILS=ON -DENABLE_CJSON_TEST=OFF
cmake --build /path/to/build-host/cJSON/build -- -j
```

2. Run `cargo test` while telling the Rust linker where to find the cJSON libraries (example assumes you built cJSON under `build-host/cJSON/build` inside the project root):

```bash
cd cjson-rs
LD_LIBRARY_PATH="$(pwd)/../build-host/cJSON/build" \
RUSTFLAGS='-L native=$(pwd)/../build-host/cJSON/build -l cjson -l cjson_utils' \
cargo test --lib
```

Notes:
- Use `-l cjson -l cjson_utils` to link the shared libraries, or link the static variants with `-l static=cjson -l static=cjson_utils` and add the directory with `-L native=...`.
- The project-level CMake already integrates `cJSON` for embedded builds; these steps let you reuse that same cJSON build artefact for running the host-side unit tests.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## References

- [cJSON Library](https://github.com/DaveGamble/cJSON)
- [RFC 6901 - JSON Pointer](https://tools.ietf.org/html/rfc6901)
- [RFC 6902 - JSON Patch](https://tools.ietf.org/html/rfc6902)
- [RFC 7386 - JSON Merge Patch](https://tools.ietf.org/html/rfc7386)
