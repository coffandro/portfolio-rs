use sdl2::rect::Rect;

use crate::{LevelData, math::Vector2};

pub struct State {
    pub pos: Vector2,
    pub dir: f32,
    pub rect: Rect,
    pub level: LevelData
}

impl Clone for State {
    fn clone(&self) -> State {
        return State {
            pos: self.pos,
            dir: self.dir,
            rect: self.rect,
            level: self.level.clone(),
        }
    }
}