use log::info;

use crate::{State, math::Vector2};

fn ccw(a: Vector2, b: Vector2, c: Vector2) -> bool {
    return (c.y - a.y) * (b.x - a.x) >
           (b.y - a.y) * (c.x - a.x);
}

fn segment_intersect(
    a: Vector2,
    b: Vector2,
    c: Vector2,
    d: Vector2
) -> bool {
    return ccw(a,c,d) != ccw(b,c,d) && ccw(a,b,c) != ccw(a,b,d);
}

pub fn attempt_move_to (
    velocity: Vector2,
    state: &mut State,
) {
    let new_pos = state.pos + velocity;
    
    for wall in state.level.walls.iter() {
        if segment_intersect(state.pos, new_pos, wall.a, wall.b) {
            return;
        }
    }

    state.pos += velocity;
}