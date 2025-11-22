watch:
	bacon watch

build-web:
	EMCC_CFLAGS="-s USE_SDL=2" \
	RUSTFLAGS="-C link-arg=-sINVOKE_RUN=0 -C link-arg=-sEXPORTED_RUNTIME_METHODS=['ccall','cwrap']" \
	cargo build --target wasm32-unknown-emscripten 