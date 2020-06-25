use {
    rmp_serde,
    serde::{
        Deserialize,
        Serialize
    },
    wasm_bindgen::prelude::*,
};

#[wasm_bindgen]
extern "C" {
    // Note: the bindings may mis-map when the order is changed.
    fn log_int_raw(s: i32);
    fn log_str_raw(offset: i32, len: i32);
    fn log_ab_raw(offset: i32, len: i32);
}

// log an integer
pub fn log_int(s: i32) {
    log_int_raw(s)
}

// log a string
pub fn log_str(s: &str) {
    // convert the string to a slice
    let slice = s.as_bytes();
    // pass the offset and len of the slice
    log_str_raw(slice.as_ptr() as i32, slice.len() as i32);
}

// Both this application and the WebAssembly file include this struct definition.
include!("../../src/model.rs");

// log a struct
pub fn log_ab(ab: &AB) {
    // serialized struct to a slice
    let slice = &rmp_serde::to_vec(ab).unwrap();
    // pass the offset and len of the slice
    log_ab_raw(slice.as_ptr() as i32, slice.len() as i32);
}

