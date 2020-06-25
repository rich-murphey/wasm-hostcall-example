default:
	wasm-pack build wasm
	cargo run
clean:
	-mv target a
	-mv wasm/target b
	-pkill rust-analyzer; pkill cargo ;rm -rf a b
	rm -f wasm/pkg Cargo.lock wasm/Cargo.lock 
