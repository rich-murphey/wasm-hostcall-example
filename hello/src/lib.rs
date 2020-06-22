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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

// #[wasm_bindgen] does not support returning Result<i32,JsValue> from
// extern fn.

#[wasm_bindgen]
extern "C" {
    fn logint(s: i32) -> i32;
    fn logstr(s: &str) -> i32;
    fn logab_(s: &[u8]) -> i32;
}

fn logab(ab: &AB) -> i32 {
    logab_(
        &rmp_serde::to_vec(ab).unwrap()
    )
}

#[wasm_bindgen]
pub fn hello() -> Result<i32,JsValue> {
    let n = logint(1234);
    let _ = logint(n);
    let n = logstr("abcd");
    let _ = logint(n);
    let _ = logab(&AB{a: 1, b: "1234 asdf".to_string()});
    Ok(4567)
}
