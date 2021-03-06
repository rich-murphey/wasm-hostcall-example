## Contents
* [WebAssembly passing Structs to Host functions](#webassembly-passing-structs-to-host-functions)
* [Prerequisites](#prerequisites)
* [Building](#building)
* [Code Samples](#code-samples)
      
## WebAssembly passing Structs to Host functions

This demo shows [WebAssembly] (Wasm) calling host application
functions, where both Wasm and application are in Rust.  In
particular, Wasm is passing references to objects that have either
static or dynamic size.

[Wasmtime] is an embedded WebAssembly virtual machine runtime.  The
Rust application uses [Wasmtime] to load and run a Rust WebAssembly
module. This demo shows the WebAssembly module calling functions in
the host application.

[Wasmtime] is new and evolving. Features to import and export
functions between WebAssembly and host will almost certainly be
enhanced. Here are some production uses:

* Fastly provides a [http_guest] API. Customers use this to deploy
  Rust web apps on edge compute.
* Cloudflare provides a Wasm Wireshark firewall API, [Wirefilter], to
  run customer Wasm firewall at the edge. Cloudflare is expanding this
  to [Web Application
  Firewall](https://blog.cloudflare.com/making-the-waf-40-faster/).

This demo is intended to show how to work within certain interim
limitations on argument types.

One limitation is, [WebAssembly] (Wasm) is 32-bit while the
application is 64-bit. Wasm pointers are a 32-bit offset in Virtual
Machine (VM) memory. To obtain a 64-bit address on the host side, Wasm
pointers must be indexed into VM memory's byte array. Fat pointers
such as &[u8] or &str are handled transparently on the WebAssembly
side; however, on the host side, they are received as two separate
arguments, the 32-bit offset and size.

An additional limitation is pointers to structs.  Passing a pointer to
a struct (e.g. &struct) requires additional code in both WebAssembly
and the host application. This demo shows examples for two kinds of
structs:
* Structs that have the Serialize trait. We
  serialize it and pass the offset and length of the serialized copy
  instead. Fields can be String, Vec and other dynamic sized types.
* Structs that have the Copy trait &mdash; a fixed size and no
  pointer fields. We pass the the offset and size of the struct itself.

There are certain trade-offs. 
* Serialization verifies the struct's field types.
* Directly passing 'Copy' structs does not, and is faster.

In both examples here, the size of the struct is verified.

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

Rust WebAssembly imports these functions from the host application to demonstrate
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
// Given a rust &str at an offset and size in caller's Wasm memory, log it to stdout.
fn log_str(caller: Caller<'_>, offset: i32, size: i32) -> Result<(), Trap> {
    let mem :Memory = mem_from(&caller)?;                 // caller's VM memory
    let slice :&[u8] = slice_from(&mem, offset, size)?; // string's byte slice
    let string :&str = std::str::from_utf8(slice)         // convert to utf-8
        .or_else(|_|Err(Trap::new("invalid utf-8")))?;
    println!("str: {}", string);                          // print the string
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
  API.  Fastly's Terrarium runs customer's WebAssembly in a edge web
  server. Customer's WebAssembly module handles specified http requests.
* Cloudflare's [Wirefilter](https://github.com/cloudflare/wirefilter), 
  runs customers Wireshark-like filters in WebAssembly on edge compute.* 
* The [Deno plugin
  API](https://github.com/denoland/deno/blob/master/core/plugin_api.rs)
  is an interface between host native Rust and either Wasm or Javascript: [Rust host plugin
  example](https://github.com/denoland/deno/blob/master/test_plugin/src/lib.rs), 
[client javascript
  example](https://github.com/denoland/deno/blob/master/test_plugin/tests/test.js),
  [blog post](https://medium.com/@enzoalbornoz/como-desenvolver-plugins-nativos-em-rust-para-deno-ecb2676ac8d3).

Suggestions and comments are welcome. Please feel free to open an
issue if you can suggest improvements, or find parts that are unclear.

[WebAssembly]: https://webassembly.org
[Wasmtime]: https://github.com/bytecodealliance/wasmtime
[RLBox]: https://plsyssec.github.io/rlbox_sandboxing_api/sphinx/
[wasm-bindgen]: https://github.com/rustwasm/wasm-bindgen
[http_guest]: https://wasm.fastlylabs.com/docs/rust/http_guest/hostcalls/index.html
[Wirefilter]: https://github.com/cloudflare/wirefilter

