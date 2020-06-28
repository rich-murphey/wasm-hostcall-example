use {
    std::marker::Sized,
    std::fmt::Debug,
    rmp_serde,
    serde::{
        Deserialize,
        Serialize
    },
    wasm_bindgen::prelude::*,
    arrayvec::ArrayString,
};

#[wasm_bindgen]
extern {
    // Note: the bindings may mis-map when the order is changed.
    fn log_ab_raw(s: &[u8]);
    fn log_cd_raw(s: &[u8]);
    pub fn log_str(s: &str);
    pub fn log_int(s: i32);
}

// Both this application and the WebAssembly file include the struct definitions.
include!("../../src/model.rs");

// log a struct
pub fn log_ab(ab: &AB) {
    // serialized struct to a slice, and pass it
    log_ab_raw(&rmp_serde::to_vec(ab).unwrap());
}

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
}

// log a struct that implements Copy and Debug
pub fn log_struct<T, LogFn>(t: &T, f: LogFn)
where T: Sized + Copy + Debug,
      LogFn: Fn(&[u8]),
{
    // serialized struct to a slice
    let slice = any_as_u8_slice::<T>(t);
    // pass the offset and len of the slice
    f(slice);
}

pub fn log_cd(cd: &CD) {
    log_struct(cd, log_cd_raw);
}
