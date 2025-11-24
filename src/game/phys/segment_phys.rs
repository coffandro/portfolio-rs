use crate::{State, Wall, consts::{RAY_MAX, RAY_RES}, math::Vector2};

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

pub fn attempt_move_to(
    velocity: Vector2,
    state: &mut State,
) {
    let new_pos = state.pos + velocity;
    
    for wall in state.level.walls.iter() {
        if segment_intersect(state.pos, new_pos, wall.segment.a, wall.segment.b) {
            return;
        }
    }

    state.pos += velocity;
}

pub struct Ray {
    pub hit: bool,
    pub pos: Vector2,
    pub collision: Wall,
    pub dist: f32
}

pub fn raycast(
    origin: Vector2,
    dir: Vector2,
    state: State,
) -> Option<Ray> {
    let mut travel = 0.0;
    let mut hit = false;
    let mut ret_ray: Option<Ray> = None;

    while travel < RAY_MAX && !hit {
        for wall in state.level.walls.iter() {
            if segment_intersect(
                origin,
                origin + (dir * travel),
                wall.segment.a,
                wall.segment.b
            ) {
                hit = true;
                ret_ray = Some(Ray {
                    hit,
                    pos: origin + (dir * travel),
                    collision: wall.clone(),
                    dist: travel
                });
            }
        }

        travel += RAY_RES;
    }

    return ret_ray;
}