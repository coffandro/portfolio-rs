pub mod math;
mod game;

#[cfg(target_family = "wasm")]
pub mod emscripten;

pub use game::*;