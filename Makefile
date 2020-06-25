default:
	wasm-pack build wasm
	cargo run
clean:
	cargo clean
	cd wasm; cargo clean
