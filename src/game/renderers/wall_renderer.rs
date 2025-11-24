use sdl2::{render::Canvas, video::Window};

use crate::consts::*;
use crate::State;
use crate::math::Vector2;

pub fn draw(
    canvas: &mut Canvas<Window>,
    state: &mut State,
) {
    canvas.set_draw_color(WHITE);
    // Draw walls
    for wall in state.level.walls.iter() {
        let _ = canvas.draw_line(
            wall.segment.a,
            wall.segment.b
        );
    }

    // Draw player
    let _ = canvas.draw_rect(state.rect);
    let _ = canvas.draw_line(
        state.pos,
        state.pos + Vector2::down().rotated(state.dir) * 25.0
    );
}
