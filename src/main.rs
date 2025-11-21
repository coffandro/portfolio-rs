mod emscripten;

use std::cell::RefCell;
use std::rc::Rc;

use portfolio::*;

// Resources
//     https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm
//     https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/

// To build locally:
//     cargo run

// To build for the web:
//     rustup target add asmjs-unknown-emscripten
//     export EMCC_CFLAGS="-s USE_SDL=2"
//     cargo build --target asmjs-unknown-emscripten && open index.html
//#![no_main]
//#[unsafe(no_mangle)]
fn main() {
    fern::Dispatch::new()
        .chain(std::io::stdout())
        .chain(std::io::stderr())
        .chain(fern::log_file("hello.txt").unwrap())
        .format(move |out, message, record| {
            out.finish(format_args!("[{}] {}", record.level(), message))
        })
        .apply()
        .unwrap();

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx
        .window("Portfolio!", 640, 480)
        .position_centered()
        .opengl()
        .build() {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let canvas = match window
        .into_canvas()
        .present_vsync()
        .build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err)
    };

    let ctx = Rc::new(RefCell::new(ctx));
    let state = setup();
    let state = Rc::new(RefCell::new(state));
    let canvas = Rc::new(RefCell::new(canvas));

    #[cfg(target_family = "wasm")]
    emscripten::set_main_loop_callback(main_loop(Rc::clone(&ctx), Rc::clone(&state), Rc::clone(&canvas)));
}