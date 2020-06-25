## Rust Hostcall Example

This example shows a way to pass strings and structures from Rust wasm code
to a Rust application that uses Wasmtime.

Three example functions in src/exports.rs are exported from the
application to Wasm:
```
fn log_int(s: i32)
fn log_str(s: &str)
fn log_ab(ab: &AB)
```
Where AB is:
```
pub struct AB {
    pub a: u32,
    pub b: String,
}
```

They are exported from the Rust application in the
[src/exports.rs](src/exports.rs) and imported into Wasm in [wasm/src/imports.rs](wasm/src/imports.rs).

The raw interface available between the host application and wasm is
limited to numeric arguments.  To pass a string, the offset and length
are passed instead.  To pass an arbitrary object, the offset and
length of a serialized copy is passed instead.

In wasm/src/lib.rs the function hello() is exported. It calls each of
the exported functions, to demonstrate all three calls.


Here is a host application function that takes a string argument.
```
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

Here is the wasm wrapper that calls the host application function.
```
pub fn log_str(s: &str) {
    // convert the string to a slice
    let slice = s.as_bytes();
    // pass the offset and len of the slice
    log_str_raw(slice.as_ptr() as i32, slice.len() as i32);
}
```
