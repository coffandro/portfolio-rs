use sdl2::rect::Rect;

use crate::{consts::LevelData, math::Vector2};

pub struct State {
    pub pos: Vector2,
    pub dir: Vector2,
    pub plane: Vector2,
    pub rect: Rect,
    pub level: LevelData
}

impl Clone for State {
    fn clone(&self) -> State {
        return State {
            pos: self.pos,
            dir: self.dir,
            plane: self.plane,
            rect: self.rect,
            level: self.level.clone(),
        }
    }
}