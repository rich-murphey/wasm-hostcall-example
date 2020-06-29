use {
    std::marker::Sized,
    std::fmt::Debug,
    rmp_serde,
    serde::{
        Deserialize,
        Serialize
    },
    arrayvec::ArrayString,
};

mod raw {
    use wasm_bindgen::prelude::wasm_bindgen;
    #[wasm_bindgen]
    extern {
        // Note: the bindings may mis-map when the order is changed.
        pub fn log_ab(s: &[u8]);
        pub fn log_cd(s: &[u8]);
        pub fn log_str(s: &str);
        pub fn log_int(s: i32);
    }
}
pub use raw::log_str;           // export these as-is
pub use raw::log_int;

// Both this application and the WebAssembly file
// include the struct definitions.
include!("../../src/models.rs");

// log a struct
pub fn log_ab(ab: &AB) {
    raw::log_ab(&rmp_serde::to_vec(ab).unwrap()); // slice containing serialized struct
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
where T:  Copy + Debug,
      LogFn: Fn(&[u8]),
{
    let slice = any_as_u8_slice::<T>(t); // slice containing struct, zero-copy
    f(slice);                  // pass the offset and len of the slice
}

pub fn log_cd(cd: &CD) {
    log_struct(cd, raw::log_cd);
}
