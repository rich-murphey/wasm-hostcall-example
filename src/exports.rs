use {
    std::marker::Sized,
    std::fmt::Debug,
    std::mem::transmute,
    wasmtime::{
        Caller,
        Func,
        Memory,
        Store,
        Trap,
    },
    serde::{
        Deserialize,
        Serialize,
    },
    arrayvec::ArrayString,
};


// for additional details about these conventions, see:
// https://github.com/bytecodealliance/wasmtime/blob/main/crates/wasmtime/src/func.rs

// Get the Memory object from the wasm caller.
fn mem_from(caller: &Caller) -> Result<Memory, Trap> {
    match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(mem)) => Ok(mem),
        _ => Err(Trap::new("failed to get caller's exported memory")),
    }
}

// get a slice at offset and length in the caller's wasm memory.
fn slice_from<'a>(mem: &'a Memory, offset: i32, length: i32) -> Result<&[u8], Trap> {
    match
        // get caller's whole wasm memory as a slice
        unsafe { mem.data_unchecked() }
        // get a sub-slice
        .get(offset as u32 as usize..(offset + length) as u32 as usize)
    {
        Some(x) => Ok(x),
        _ => Err(Trap::new("pointer or length out of range"))
    }
}

// transmute a slice of caller's wasm memory to a struct reference.
fn struct_from<T>(mem: &Memory, offset: i32, length: i32) -> Result<&T, Trap> {
    if length as u32 as usize != std::mem::size_of::<T>() {
        return Err(Trap::new("struct size not equal to slice length"));
    }
    Ok(
        unsafe {
            transmute::<*const u8, &T>(
                slice_from(&mem, offset, length)? // Err if offset or length out of range
                .as_ptr()
            )
        }
    )
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

// Given a struct that implments 'Copy' at an offset and
// length in caller's wasm memory, log it to stdout.
fn log_struct<T>(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap>
where T: Sized + Copy + Debug
{
    // get the caller's wasm memory
    let mem :Memory = mem_from(&caller)?;
    // get ref to struct in slice at the given offset and length
    let t :&T = struct_from(&mem, offset, length)?;
    println!("struct: {:?}", t);
    Ok(())
}

pub fn get_funcs(store: &Store) -> Vec<wasmtime::Extern> {
    vec![
        // Note: the bindings may mis-map when the order is changed.
        Func::wrap(store, log_ab).into(),
        Func::wrap(store, log_struct::<CD>).into(),
        Func::wrap(store, log_str).into(),
        Func::wrap(store, log_int).into(),
    ]
}
