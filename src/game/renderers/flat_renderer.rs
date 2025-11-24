use sdl2::{render::Canvas, video::Window};

use crate::consts::*;
use crate::State;

pub fn draw(
    canvas: &mut Canvas<Window>,
    state: &mut State,
) {
    canvas.set_draw_color(WHITE);
    // Draw walls
    for wall in state.level.walls.iter() {
        let _ = canvas.draw_line(
            wall.a.to_point(),
            wall.b.to_point()
        );
    }

    // Draw player
    let _ = canvas.draw_rect(state.rect);
    let _ = canvas.draw_line(
        state.pos.to_point(),
        (state.pos + state.dir * 25.0).to_point()
    );
}
