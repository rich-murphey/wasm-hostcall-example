default:
	wasm-pack build
clean:
	pkill rls; sleep 1;cargo clean
	rm -rf pkg


# --release
# -t no-modules
#	wasm-nm -z ./pkg/hello_bg.wasm |sort -n |rustfilt
#	wasm-nm -z ./target/wasm32-unknown-unknown/release/hello.wasm |sort -n |rustfilt
#	twiggy top -n 40 ./pkg/hello_bg.wasm
#	cp ./pkg/hello_bg.wasm ../src/hello.wasm
# 	~/.wasmtime/bin/wasmtime run --invoke hello \
# ./pkg/hello_bg.wasm 

# ./target/wasm32-unknown-unknown/release/hello.wasm

