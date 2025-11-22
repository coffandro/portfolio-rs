use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::game::renderers::*;
use crate::consts::*;
use crate::math::Vector2;
use super::state::State;

pub fn setup(level: LevelData) -> State {
    let state = State {
        pos: level.player.pos * GRID_SIZE,
        dir: -level.player.dir,
        plane: Vector2::zero(),
        rect: Rect::new(0, 0, 10, 10),
        level: level
    };

    return state;
}

fn rotate(rot: f64, state: &mut State) {
    let rot_cos = rot.cos() as f32;
    let rot_sin = rot.sin() as f32;

    state.dir.x = (state.dir.x * rot_cos) - (state.dir.y * rot_sin);
    state.dir.y = (state.dir.x * rot_sin) + (state.dir.y * rot_cos);
    state.plane.x = (state.plane.x * rot_cos) - (state.plane.y * rot_sin);
    state.plane.y = (state.plane.x * rot_sin) + (state.plane.y * rot_cos);

    state.dir.normalize();
    state.plane.normalize();
}

fn process_input(
    e: &sdl2::EventPump,
    v: &mut Vector2,
    state: &mut State
) {
    let key_state = e.keyboard_state();
    
    if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
        v.y += 1.0;
    }

    if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
        v.y -= 1.0;
    }

    if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
        v.x += 1.0;
    }

    if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
        v.x -= 1.0;
    }

    if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::Q) {
        rotate(-3.0 * 0.016, state);
    }

    if key_state.is_scancode_pressed(sdl2::keyboard::Scancode::E) {
        rotate(3.0 * 0.016, state);
    }

    v.normalize();
}

pub fn main_loop(
    ctx: Rc<RefCell<Sdl>>, 
    rstate: Rc<RefCell<State>>,
    rcanvas: Rc<RefCell<Canvas<Window>>>,
) -> impl FnMut() {
    let events = ctx.borrow_mut().event_pump().unwrap();
    
    move || {
        let mut state = rstate.borrow_mut();
        let mut canvas = rcanvas.borrow_mut();
        let mut vel = Vector2::zero();

        process_input(&events, &mut vel, &mut state);

        state.pos += vel;
        state.rect.x = (state.pos.x as i32) - state.rect.w/2;
        state.rect.y = (state.pos.y as i32) - state.rect.h/2;

        canvas.set_draw_color(BLACK);
        canvas.clear();

        draw(canvas.deref_mut(), state.deref_mut());
        
        canvas.present();
    }

}