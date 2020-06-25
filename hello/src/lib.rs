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

#[derive(Debug, Serialize, Deserialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

extern "C" {
    // the bindings break when the order is changed.
    fn log_str(offset: i32, len: i32);
    fn log_int(s: i32);
    fn log_ab(offset: i32, len: i32);
}

fn logint(s: i32) {
     unsafe {
        log_int(s)
    }
}

fn logab(ab: &AB) {
     unsafe {
         let slice = &rmp_serde::to_vec(ab).unwrap();
         log_ab(slice.as_ptr() as i32, slice.len() as i32);
    }
}

fn logstr(s: &str) {
    unsafe {
        let slice = s.as_bytes();
        log_str(slice.as_ptr() as i32, slice.len() as i32);
    }
}

#[wasm_bindgen]
pub fn hello() -> Result<i32,JsValue> {
    logstr("abcd 1234");
    logint(1234);
    logab(&AB{a: 1, b: "1234".to_string()});
    Ok(4567)
}
