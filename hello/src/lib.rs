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

extern {
    // the bindings break if you change the order.
    #[allow(improper_ctypes)]
    fn logstr_(a: &[u8]);
    fn logint_(s: i32);
    #[allow(improper_ctypes)]
    fn logab_ (s: &[u8]);
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
