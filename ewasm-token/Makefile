WAT_ARGS ?= --fold-exprs --inline-exports --generate-names

all:
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/ewasm_token.wasm ewasm_token-bloated.wasm
	wasm-gc ewasm_token-bloated.wasm ewasm_token.wasm
	chisel ewasm_token.wasm ewasm_token.wasm
	wasm-dis ewasm_token.wasm > ewasm_token.wat
	wasm-dis ewasm_token-bloated.wasm > ewasm_token-bloated.wat
