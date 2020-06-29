## Contents
* [WebAssembly calling Host functions](#webassembly-calling-host-functions)
* [Prerequisites](#prerequisites)
* [Building](#building)
* [Code Samples](#code-samples)
      
## WebAssembly calling Host functions

This demo shows how a Rust [WebAssembly] (Wasm) module can call
functions in a Rust application.  The Rust application uses [Wasmtime]
to load and run a Rust WebAssembly library, also written in Rust. The
WebAssembly library calls functions in the the Rust host application.

[Wasmtime] is quite new and evolving, especially new features to
import and export functions between WebAssembly and host.  So, this
information could be obsoleted or changed as new features are
released. This demo is intended to show how things currently work, and
certain interim limitations on argument types.

One limitation is, WebAssembly (Wasm) is 32-bit while the application
is 64-bit. Wasm pointers are a 32-bit offset in a byte array of Wasm
Virtual Machine memory. To obtain a 64-bit address on the host side,
Wasm pointers are indexed into the Wasm memory. Fat pointers function
arguments such as &[u8] or &str parameter are handled transparently on
the WebAssembly side, but on the host side, they are received as two
separate arguments, the 32-bit offset and length.

An additional limitation is pointers to structs.  Passing a pointer to
a struct requires additional code for both WebAssembly and host. We
have examples for two kinds of structs:
* structs that have the Copy trait -- are a fixed size and contain no
  pointers. We pass the the offset and size of the struct.
* structs that have the Serialize trait -- can be serialized. We
  serialized it and pass the offset and length of the serialized copy
  instead. Members can be String, Vec and other dynamic sized types.

Note: this does not address security issues, which motivate additional
validation and sand-boxing techniques such as [RLBox].

## Prerequisites

To build this demo, first 
[install rust](https://www.rust-lang.org/tools/install), then add features:

```sh
rustup target add wasm32-wasi
cargo install wasm-pack
```

## Building
After the above, clone this project:
```sh
git clone https://github.com/rich-murphey/wasm-hostcall-example.git
cd wasm-hostcall-example
```
Then build the WebAssembly module:
```sh
wasm-pack build wasm
```
Then build and run the application:
```sh
cargo run
```
## Code Samples

The the WebAssembly imports functions from the Rust host to demonstrate
passing various argument types:
```rust
fn log_int(s: i32)   // passes an integer
fn log_str(s: &str)  // passes pointer and size, zero-copy.
fn log_ab(ab: &AB)   // passes pointer and size of a serialized copy
fn log_cd(cd: &CD)   // passes pointer and size of a struct, zero-copy.

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

The WebAssembly (Wasm) function hello() in [wasm/src/lib.rs](wasm/src/lib.rs) calls the above functions.
```rust
pub fn hello() -> Result<i32,JsValue> {
    log_int(1234);
    log_str("Hello World!");
    log_ab(&AB{a: 1234, b: "abcd".to_string()});
    log_cd(&CD::from(1234, "hello world"));
    Ok(4567)
}
```

The WebAssembly side of the API is defined in
[wasm/src/imports.rs](wasm/src/imports.rs).  Note that log_int() and
log_str() do not need any additional conversion on the WebAssembly side.


The host (application) side of the API is defined in [src/exports.rs](src/exports.rs):
```rust
// Given a rust &str at an offset and length in caller's Wasm memory, log it to stdout.
fn log_str(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap> {
    let mem :Memory = mem_from(&caller)?;                  // get caller's Wasm memory
    let slice :&[u8] = slice_from(&mem, offset, length)?;  // get string's byte slice
    let string :&str = std::str::from_utf8(slice)          // convert to utf-8
        .or_else(|_|Err(Trap::new("invalid utf-8")))?;
    println!("str: {}", string);                           // print the string
    Ok(())
}
```

See [exports.rs](src/exports.rs) and [imports.rs](wasm/src/imports.rs)
for the corresponding code for the other functions in the API.

## Acknowledgments
* [The Bytecode Alliance](https://bytecodealliance.org) hosts
  WebAssembly, Wasmtime, Cargo Wasi, Wasi, Lucet and others.
* [Wasmtime](https://github.com/bytecodealliance/wasmtime) implements a
  WebAssembly run-time (virtual machine).
* [Cargo Wasi](https://github.com/bytecodealliance/cargo-wasi) is a
  tool for compiling rust modules to WebAssembly.
* [WebAssembly System
  Interface](https://github.com/bytecodealliance/wasi) is analogous to
  parts of libc.
* Fastly's
[http_guest](https://wasm.fastlylabs.com/docs/rust/http_guest/hostcalls/index.html)
API.  The Rust application (host) is a web server, while the WebAssembly
module handles specific http requests.

Suggestions and comments are welcome. Please feel free to open an
issue if you can suggest improvements, or find parts that are unclear.

[WebAssembly]: https://webassembly.org
[Wasmtime]: https://github.com/bytecodealliance/wasmtime
[RLBox]: https://plsyssec.github.io/rlbox_sandboxing_api/sphinx/
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
