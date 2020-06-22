default:
	wasm-pack build hello
	cargo run
clean:
	pkill rust-analyzer ;cargo clean


# cd wasi; cargo wasi build --release
# cp ./wasi/target/wasm32-wasi/release/wasi.wasm src/
# pwd

# ERROR[E0277]: the trait bound `

# extern "C" fn(<i32 as wasm_bindgen::convert::traits::FromWasmAbi>::Abi)
#  ->  <i32 as wasm_bindgen::convert::traits::ReturnWasmAbi>::Abi {__wasm_bindgen_generated_logint}: wasmtime::func::IntoFunc<_, _>

# ` is not satisfied
#   --> src/main.rs:33:32
#    |
# 33 |         .func("wbg", "logint", __wasm_bindgen_generated_logint)?
#    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `wasmtime::func::IntoFunc<_, _>` is not implemented for `extern "C" fn(<i32 as wasm_bindgen::convert::traits::FromWasmAbi>::Abi) -> <i32 as wasm_bindgen::convert::traits::ReturnWasmAbi>::Abi {__wasm_bindgen_generated_logint}`

