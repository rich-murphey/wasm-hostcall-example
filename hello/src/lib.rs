use {
    rmp_serde,
    serde::Serialize,
    wasm_bindgen::prelude::*,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct AB {
    pub a: u32,
    pub b: String,
}

// #[wasm_bindgen] does not support returning Result<i32,JsValue> from
// extern fn.

#[no_mangle]
extern "C" {
    fn logint_(s: i32);
    fn logab_ (s: &[u8]);
    fn logstr_(s: &[u8]);
}

fn logint(s: i32) {
     unsafe {
        logint_(s)
    }
}
fn logab(ab: &AB) {
     unsafe {
        logab_(&rmp_serde::to_vec(ab).unwrap())
    }
}
fn logstr(s: &str) {
    unsafe {
        logstr_(s.as_bytes())
    }
}

#[wasm_bindgen]
pub fn hello() -> Result<i32,JsValue> {
    logstr("abcd");
    logint(1234);
    logab(&AB{a: 1, b: "1234 asdf".to_string()});
    Ok(4567)
}
