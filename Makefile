.PHONY: setup install run
setup:
	rustup target add wasm32-wasi
install:
	curl https://get.wasmer.io -sSfL | sh
run:
	cargo build --target wasm32-wasi
	wasmer ./target/wasm32-wasi/debug/wasm_sample.wasm
build:
	make setup
	make run