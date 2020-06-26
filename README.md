## Contents
* [About This Project](#about-this-project)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Building](#building)
* [Code Excerpts](#code-excerpts)
      
## About This Project

This is an example of how to export and import functions between a Rust
application that loads WebAssembly, and Rust WebAssembly it loads.

Wasmtime is quite new and evolving, especially new features to
import/export functions between Wasm and host.  This demo is intended
to show some ways to work with interim limitations on argument types.

Currently, in the raw interface between the host and WebAssembly,
arguments are limited to integer and floating point numbers. So, in
order to pass a string, the offset and length are passed instead.  To
pass an arbitrary object, the offset and length of a serialized copy
is passed instead. To pass a fixed size struct that contains no
pointers (i.e. implements Copy), the offset and size is passed instead. 

Suggestions and comments are welcome. Please feel to open an issue if
you can suggest better ways of writing these, or find parts that are
unclear.

## Getting Started

Here's how to build this example.

### Prerequisites

[Install rust](https://www.rust-lang.org/tools/install), then add features:

```sh
rustup target add wasm32-wasi
cargo install wasm-pack
```

### Building
After the above, clone this project:
```sh
git clone https://github.com/rich-murphey/wasm-hostcall-example.git
```
Then build the Wasm module:
```sh
wasm-pack build wasm
```
Then build and run the application:
```
cargo run
```
## Code Excerpts

The host (application) exports the following functions to demonstrate
passing arguments that are:
* integers
* strings
* serializable structs, and
* zero-copy fixed-sized structs.

Here is the interface in Wasm Rust code:
```rust
fn log_int(s: i32);
fn log_str(s: &str);
fn log_ab(ab: &AB); // serialized
fn log_cd(cd: &CD); // zero copy

#[derive(Debug, Serialize, Deserialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

#[derive(Debug, Copy, Clone)]
pub struct CD {
    pub c: i32,
    pub d: ArrayString::<[u8; CD_N]>,
}
```

The Wasm function hello() calls the above functions, to demonstrate.
[wasm/src/lib.rs](wasm/src/lib.rs), calls them:
```rust
pub fn hello() -> Result<i32,JsValue> {
    log_str("Hello World!");
    log_int(1234);
    log_ab(&AB{a: 1234, b: "abcd".to_string()});
    log_cd(&CD::from(1234, "hello world"));
    Ok(4567)
}
```

The Wasm side of the API are defined in
[wasm/src/imports.rs](wasm/src/imports.rs), such as:
```rust
pub fn log_str(s: &str) {
    // convert the string to a slice (&[u8]}, and pass it to the host.
    // Note: When Wasm passes &[u8], the host recieves offset: i32, length: i32.
    log_str_raw(s.as_bytes());
}
```

The host (application) side of the API are defined in [src/exports.rs](src/exports.rs):
```rust
// Given a rust &str at an offset and length in caller's Wasm memory, log it to stdout.
fn log_str_raw(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap> {
    // get the caller's Wasm memory
    let mem :Memory = mem_from(&caller)?;
    // get a slice at the given offset and length
    let slice :&[u8] = slice_from(&mem, offset, length)?;
    // get a string from the slice
    let string :&str = std::str::from_utf8(slice)
        .or_else(|_|Err(Trap::new("invalid utf-8")))?;
    // log the string
    println!("str: {}", string);
    Ok(())
}
```

See [exports.rs](src/exports.rs) and [imports.rs](wasm/src/imports.rs)
for the corresponding code for `fn log_ab(ab: &AB)`.

## Acknowledgments
* [The Bytecode Alliance](https://bytecodealliance.org)
* [Wasmtime](https://github.com/bytecodealliance/wasmtime)
* [Cargo Wasi](https://github.com/bytecodealliance/cargo-wasi)
* [WebAssembly System Interface](https://github.com/bytecodealliance/wasi)
