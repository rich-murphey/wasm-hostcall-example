use {
    anyhow::{
        format_err,
        Context,
        Result
    },
    wasmtime::{
        Func,
        Instance, 
        Module,
        Memory,
        Store,
    },
    serde::{
        Deserialize,
        Serialize
    },
};

static mut MEM: Option<Memory> = None;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

fn slice_from<'a>(offset: i32, length: i32) -> &'a [u8] {
    unsafe {
        &MEM.as_ref().unwrap()
            .data_unchecked()[offset as usize..][..length as usize]
    }
}

fn str_from<'a>(offset: i32, length: i32) -> &'a str {
    let slice = slice_from(offset, length);
    std::str::from_utf8(slice).unwrap()
}

fn struct_from_rmps<'a, T: Deserialize::<'a>>(offset: i32, length: i32) -> T {
    let slice = slice_from(offset, length);
    rmp_serde::from_slice(slice).unwrap()
}

pub fn logint(s: i32) {
    println!("int: {}", s);
}

pub fn logab(offset: i32, length: i32) {
    println!("struct: {:?}", struct_from_rmps::<AB>(offset, length));
}

pub fn logstr(offset: i32, length: i32) {
    println!("str: {}", str_from(offset, length));
}

fn main() -> Result<()> {
    let wasm_file = "hello/pkg/hello_bg.wasm";
    let store = Store::default();
    let module = Module::from_file(store.engine(), wasm_file)?;
    let instance = Instance::new(&store, &module, &[
        // the bindings break if you change the order.
        Func::wrap(&store, logstr).into(),
        Func::wrap(&store, logint).into(),
        Func::wrap(&store, logab).into(),
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
