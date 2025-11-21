use sdl2::rect::Rect;

use crate::math::Vector2;

pub struct State {
    pub pos: Vector2,
    pub dir: Vector2,
    pub plane: Vector2,
    pub rect: Rect,
}