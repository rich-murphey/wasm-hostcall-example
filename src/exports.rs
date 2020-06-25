use {
    wasmtime::{
        Caller,
        Func,
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

pub fn get_funcs(store: &Store) -> Vec<wasmtime::Extern> {
    vec![
        // Note: the bindings may mis-map when the order is changed.
        Func::wrap(store, log_int).into(),
        Func::wrap(store, log_str).into(),
        Func::wrap(store, log_ab).into(),
    ]
}
