///! This example shows how to import functions from a host Rust
///! application to a Rust wasm module.  

///! Example functions show:
///! - passing a &str from wasm to the host
///! - passing a serde encoded struct from wasm to the host.

///! You can run this example using 'make'.

mod exports;
use {
    anyhow::{
        format_err,
        Context,
    },
};

fn main() -> anyhow::Result<()> {
    // load the WebAssembly file, hello/pkg/hello_bg.wasm, and attach
    // it to a store containing the wasm memory, etc.
    let store = wasmtime::Store::default();
    let wasm_file = "hello/pkg/hello_bg.wasm";
    let module = wasmtime::Module::from_file(store.engine(), wasm_file)
        .context(wasm_file)?;

    // create a Wasm runtime with the WebAssembly file and exports.
    let instance = wasmtime::Instance::new(&store, &module, &exports::get_funcs(&store))
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
