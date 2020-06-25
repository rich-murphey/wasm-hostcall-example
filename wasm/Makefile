default:
	wasm-pack build
clean:
	pkill rls; sleep 1;cargo clean
	rm -rf pkg
expand:
	cargo +nightly rustc --profile=check -- -Zunstable-options --pretty=expanded|rustfmt
