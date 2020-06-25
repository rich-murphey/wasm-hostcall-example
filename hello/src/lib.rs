use {
    rmp_serde,
    serde::{
        Deserialize,
        Serialize
    },
    wasm_bindgen::prelude::*,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Note: the bindings may mis-map when the order is changed.
    fn log_str(offset: i32, len: i32);
    fn log_int(s: i32);
    fn log_ab(offset: i32, len: i32);
}

// log an integer
fn logint(s: i32) {
    log_int(s)
}

// log a string
fn logstr(s: &str) {
    // convert the string to a slice
    let slice = s.as_bytes();
    // pass the offset and len of the slice
    log_str(slice.as_ptr() as i32, slice.len() as i32);
}

// Both this application and the WebAssembly file include this struct definition.
include!("../../src/model.rs");

// log a struct
fn logab(ab: &AB) {
    // serialized struct to a slice
    let slice = &rmp_serde::to_vec(ab).unwrap();
    // pass the offset and len of the slice
    log_ab(slice.as_ptr() as i32, slice.len() as i32);
}

#[wasm_bindgen]
pub fn hello() -> Result<i32,JsValue> {
    logstr("abcd 1234");
    logint(1234);
    logab(&AB{a: 1, b: "1234".to_string()});
    Ok(4567)
}
