///! This example shows how to import functions from a host Rust
///! application to a Rust wasm module.  

///! Example functions show:
///! - passing a &str from wasm to the host
///! - passing a serde encoded struct from wasm to the host.

///! You can run this example using 'make'.

use {
    anyhow::{
        format_err,
        Context,
    },
    wasmtime::{
        Caller,
        Func,
        Module,
        Memory,
        Store,
        Trap,
    },
    serde::{
        Deserialize,
        Serialize
    },
};

// Given an offset and length in the caller's wasm memory, return a slice.
fn slice_from<'a>(mem: &'a Memory, offset: i32, length: i32) -> Result<&[u8], Trap> {
    match unsafe {
        mem.data_unchecked()    // get a slice of the wams memory
            .get(offset as u32 as usize..) // starting at offset
            .and_then(|arr| arr.get(..length as u32 as usize)) // with given length
    } {
        Some(data) => Ok(data),
        None => Err(Trap::new("pointer/length out of bounds")),
    }
}
// Get the Memory object from the wasm caller.
fn mem_from(caller: &Caller) -> Result<Memory, Trap> {
    match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(mem)) => Ok(mem),
        _ => Err(Trap::new("failed to get caller's exported memory")),
    }
}

fn log_int(i: i32) {
    // log the given integer
    println!("int: {}", i);
}

// Given a rust &str at an offset and length in caller's wasm memory, log it to stdout.
fn log_str(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap> {
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

// Both this application and the WebAssembly file include this struct definition.
include!("model.rs");

// Given a serialized Rust Message Pack struct at an offset and
// length in caller's wasm memory, log it to stdout.
fn log_ab(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap> {
    // get the caller's wasm memory
    let mem :Memory = mem_from(&caller)?;
    // get a slice at the given offset and length
    let slice :&[u8] = slice_from(&mem, offset, length)?;
    // deserialize a struct from the slice
    let ab :AB = rmp_serde::from_slice(slice)
        .or_else(|e|Err(Trap::new(&format!("invalid RMP data: {:?}", e))))?;
    // print the struct
    println!("struct: {:?}", ab);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // load the WebAssembly file, hello/pkg/hello_bg.wasm, and attach
    // it to a store containing the wasm memory, etc.
    let store = Store::default();
    let wasm_file = "hello/pkg/hello_bg.wasm";
    let module = Module::from_file(store.engine(), wasm_file)
        .context(wasm_file)?;

    let exports = [
        // Note: the bindings may mis-map when the order is changed.
        Func::wrap(&store, log_str).into(),
        Func::wrap(&store, log_int).into(),
        Func::wrap(&store, log_ab).into(),
    ];
    // create a Wasm runtime with the WebAssembly file and exports.
    let instance = wasmtime::Instance::new(&store, &module, &exports)
        .context("failed to create wasmtime instance")?;
    
    // import 'fn hello()' from the WebAssembly file.
    let func = instance
        .get_func("hello")
        .ok_or(format_err!("failed to find export: hello()"))?
        .get0::<i32>().context("failed to get fn hello()")?;

    // call the Wasm 'fn hello()'.
    println!("calling hello():");
    let res = func();
    // print the result of the call.
    println!("result of hello(): {:?}", res);

    Ok(())
}
