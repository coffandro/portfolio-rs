use sdl2::pixels::Color;

use serde::Deserialize;
use crate::math::{Vector2, Segment};

pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const WHITE: Color = Color::RGB(255, 255, 255);
pub const GRID_SIZE: i32 = 16;
pub const PLAYER_SIZE: u32 = 10;

#[derive(Deserialize, Debug, Clone)]
pub struct LevelPlayer {
    pub pos: Vector2,
    pub dir: Vector2
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub walls: Vec<Segment>,
    pub textures: Vec<String>,
    pub player: LevelPlayer
}

impl Clone for LevelData {
    fn clone(&self) -> Self {
        return LevelData {
            walls: self.walls.clone(),
            textures: self.textures.clone(),
            player: self.player.clone()
        }
    }
}