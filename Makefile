watch:
	bacon watch

build-web:
	EMCC_CFLAGS="-s USE_SDL=2" \
	cargo build --target wasm32-unknown-emscripten 