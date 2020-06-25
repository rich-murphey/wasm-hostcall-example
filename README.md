## Rust Hostcall Example

This example shows how to export and import functions between a Rust
application that loads WebAssembly, and Rust WebAssembly code.

Note: the raw interface between the host and wasm has certain
limitations.  Arguments are limited to interger and floating point
numbers. This example shows one way to adapt to these limitations.

In order to pass a string, the offset and length are passed instead.
To pass an arbitrary object, the offset and length of a serialized
copy is passed instead.

The host (application) exports the following fuctions to demonstrate passing
intergers, strings and structs.
```rust
fn log_int(s: i32)
fn log_str(s: &str)
fn log_ab(ab: &AB)

pub struct AB {
    pub a: u32,
    pub b: String,
}
```

To demonstrate this, the Wasm module,
[wasm/src/lib.rs](wasm/src/lib.rs), calls them:
```rust
pub fn hello() -> Result<i32,JsValue> {
    log_str("Hello World!");
    log_int(1234);
    log_ab(&AB{a: 1234, b: "abcd".to_string()});
    Ok(4567)
}
```

The three functions funcions are defined in [wasm/src/imports.rs](wasm/src/imports.rs):
```rust
pub fn log_str(s: &str) {
    // convert the string to a slice
    let slice = s.as_bytes();
    // pass the offset and len of the slice
    log_str_raw(slice.as_ptr() as i32, slice.len() as i32);
}
```

They, in turn, call the raw host (application) interface defined in [src/exports.rs](src/exports.rs):
```rust
// Given a rust &str at an offset and length in caller's wasm memory, log it to stdout.
fn log_str_raw(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap> {
    // get the caller's wasm memory
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

Suggestions and comments are welcome. Plees feel to open an issue if
you can suggest better ways of writing these, or find parts that are
unclear.
