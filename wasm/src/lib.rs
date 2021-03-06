mod imports;
use {
    wasm_bindgen::prelude::*,
    imports::*,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello() -> Result<i32,JsValue> {
    log_int(1234);
    log_str("Hello World!");
    log_ab(&AB{a: 1234, b: "abcd".to_string()});
    log_cd(&CD::from(1234, "hello world"));
    Ok(4567)
}
