mod emscripten;

use std::cell::RefCell;
use std::rc::Rc;

use log::info;
use portfolio::LevelData;
use portfolio::*;
use sdl2::libc;
use once_cell::sync::OnceCell;

static STATE: OnceCell<State> = OnceCell::new();

#[no_mangle]
pub extern "C" fn set_level_data(ptr: *const libc::c_char) {
    let s = unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_str()
        .unwrap()
        .to_string();

    // deserialize JSON
    let data: LevelData = serde_json::from_str(&s).unwrap();

    STATE.set(setup(data)).ok();
}

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
    let canvas = Rc::new(RefCell::new(canvas));
    let state = STATE.get().expect("State not initialized yet!");
    let state = Rc::new(RefCell::new(state.clone()));

    info!("Starting mainloop");

    emscripten::set_main_loop_callback(
        main_loop(
            Rc::clone(&ctx),
            Rc::clone(&state),
            Rc::clone(&canvas)
        )
    );
}