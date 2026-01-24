# cjson-binding

Safe Rust bindings for the [cJSON](https://github.com/DaveGamble/cJSON) library - a lightweight JSON parser in C.

## Overview

`cjson-binding` provides idiomatic, safe Rust wrappers around the cJSON C library, offering:

- **Safe API**: Memory-safe wrappers with automatic resource management (RAII)
- **Type-safe operations**: Strong typing with `Result` types for error handling
- **JSON Pointer support (RFC6901)**: Navigate JSON documents using JSON Pointer syntax
- **JSON Patch support (RFC6902)**: Generate and apply JSON patches
- **JSON Merge Patch support (RFC7386)**: Generate and apply merge patches
- **no_std compatible**: Suitable for embedded systems with built-in allocator and panic handler

## Embedded & no_std Support

The library is designed for embedded systems and supports `no_std` environments:

- **Global Allocator**: Uses C's `malloc`/`free` via FFI
- **Default Panic Handler**: Provides a simple infinite loop panic handler
- **Custom Panic Handler**: Use the `disable_panic` feature to provide your own

### Cargo Features

- **`std`**: Enables standard library support (required for tests)
- **`disable_panic`**: Disables both the default allocator and panic handler, allowing you to provide your own

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
