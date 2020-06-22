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

fn mem_str<'a>(offset: i32, length: i32) -> &'a str {
    let bytes = unsafe {
        &MEM.as_ref().unwrap()
            .data_unchecked()[offset as usize..][..length as usize]
    };
    std::str::from_utf8(bytes).unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

fn mem_struct<'a, T: Deserialize::<'a>>(offset: i32, length: i32) -> T {
    let v: T = rmp_serde::from_slice(
        unsafe {
            &MEM.as_ref().unwrap()
                .data_unchecked()[offset as usize..][..length as usize]
        }
    ).unwrap();
    v
}

pub fn logint(s: i32) -> i32 {
    println!("int: {}", s);
    2345
}
pub fn logstr(offset: i32, length: i32) -> i32 {
    println!("str: {}", mem_str(offset, length));
    3456
}
pub fn logab(offset: i32, length: i32) -> i32 {
    println!("struct: {:?}", mem_struct::<AB>(offset, length));
    3456
}

fn main() -> Result<()> {
    let wasm_file = "hello/pkg/hello_bg.wasm";
    let store = Store::default();
    let module = Module::from_file(store.engine(), wasm_file)?;
    let instance = Instance::new(&store, &module, &[
        Func::wrap(&store, logint).into(),
        Func::wrap(&store, logstr).into(),
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
