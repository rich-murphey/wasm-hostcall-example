use anyhow::{format_err, Context, Result};
use wasmtime::{
    Func,
    Instance, 
    // Linker,
    Module,
    Memory,
    Store,
    // Trap
};
// use wasmtime_wasi::{Wasi, WasiCtx};
// use wasm_bindgen::prelude::*;
static mut MEM: Option<Memory> = None;
fn mem_str<'a>(s: i32, l: i32) -> &'a str {
    let bytes = unsafe {
        &MEM.as_ref().unwrap()
            .data_unchecked()[s as usize..][..l as usize]
    };
    std::str::from_utf8(bytes).unwrap()
}

pub fn logint(s: i32) -> i32 {
    println!("int: {}", s);
    2345
}
pub fn logstring(s: i32, l: i32) -> i32 {
    // println!("len: {}", l);
    println!("str: {}", mem_str(s, l));
    3456
}

pub fn run(wasm_file: &str) -> Result<()> {
    let store = Store::default();
    let module = Module::from_file(store.engine(), wasm_file)?;
    let instance = Instance::new(&store, &module, &[
        Func::wrap(&store, logint).into(),
        Func::wrap(&store, logstring).into(),
    ])?;
    unsafe {
        MEM = Some(instance
                   .get_memory("memory")
                   .ok_or(format_err!("failed to find `memory` export"))?);
    }    
    let func = instance
        .get_func("hello")
        .ok_or(format_err!("failed to find export: hello()"))?
        .get0::<i32>().context("get0 hello")?;
    println!("calling hello():");
    let res = func();
    println!("result of hello(): {:?}", res);
    println!("done.");
    Ok(())
}

pub fn main() -> Result<()> {
    run("hello/pkg/hello_bg.wasm")?;
    Ok(())
}
