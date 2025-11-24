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
            wall.segment.a.to_point(),
            wall.segment.b.to_point()
        );
    }

    // Draw player
    let _ = canvas.draw_rect(state.rect);
}
