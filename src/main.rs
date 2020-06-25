use {
    anyhow::{
        format_err,
        Context,
    },
    wasmtime::{
        Caller,
        Extern,
        Func,
        Instance, 
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

fn slice_from<'a>(mem: &'a Memory, offset: i32, length: i32) -> Result<&[u8], Trap> {
    match unsafe {
        mem.data_unchecked()
            .get(offset as u32 as usize..)
            .and_then(|arr| arr.get(..length as u32 as usize))
    } {
        Some(data) => Ok(data),
        None => Err(Trap::new("pointer/length out of bounds")),
    }
}
fn mem_from(caller: &Caller) -> Result<Memory, Trap> {
    match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => Ok(mem),
        _ => Err(Trap::new("failed to get caller's exported memory")),
    }
}

fn log_int(i: i32) {
    println!("int: {}", i);
}

fn log_str(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap> {
    let mem = mem_from(&caller)?;
    let slice = slice_from(&mem, offset, length)?;
    let string = std::str::from_utf8(slice)
        .or_else(|_|Err(Trap::new("invalid utf-8")))?;
    println!("str: {}", string);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

fn log_ab(caller: Caller<'_>, offset: i32, length: i32) -> Result<(), Trap> {
    let mem = mem_from(&caller)?;
    let slice = slice_from(&mem, offset, length)?;
    let ab: AB = rmp_serde::from_slice(slice)
        .or_else(|e|Err(Trap::new(&format!("invalid RMP data: {:?}", e))))?;
    println!("struct: {:?}", ab);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let wasm_file = "hello/pkg/hello_bg.wasm";
    let store = Store::default();
    let module = Module::from_file(store.engine(), wasm_file)?;

    let instance = Instance::new(&store, &module, &[
        // the bindings break when the order is changed.
        Func::wrap(&store, log_str).into(),
        Func::wrap(&store, log_int).into(),
        Func::wrap(&store, log_ab).into(),
    ])?;
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
