use serde::Deserialize;

use crate::math::{Segment, Vector2};

#[derive(Deserialize, Debug, Clone)]
pub struct LevelPlayer {
    pub pos: Vector2,
    pub dir: f32
}

#[derive(Deserialize, Debug, Clone)]
pub struct Wall {
    pub segment: Segment,
    pub texture: i32,
    pub link: i32
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub walls: Vec<Wall>,
    pub textures: Vec<String>,
    pub links: Vec<String>,
    pub player: LevelPlayer
}

impl Clone for LevelData {
    fn clone(&self) -> Self {
        return LevelData {
            walls: self.walls.clone(),
            textures: self.textures.clone(),
            links: self.links.clone(),
            player: self.player.clone()
        }
    }
}