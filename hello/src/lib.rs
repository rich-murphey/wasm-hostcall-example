mod imports;
use {
    wasm_bindgen::prelude::*,
    imports::*,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello() -> Result<i32,JsValue> {
    log_str("abcd 1234");
    log_int(1234);
    log_ab(&AB{a: 1, b: "1234".to_string()});
    Ok(4567)
}
