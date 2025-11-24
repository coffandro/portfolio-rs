use sdl2::pixels::Color;

pub const BLACK: Color = Color::RGB(0, 0, 0);
pub const RED: Color = Color::RGB(255, 0, 0);
pub const GREEN: Color = Color::RGB(0, 255, 0);
pub const BLUE: Color = Color::RGB(0, 0, 255);
pub const WHITE: Color = Color::RGB(255, 255, 255);

pub const GRID_SIZE: i32 = 16;
pub const PLAYER_SIZE: u32 = 10;

pub const RAY_RES: f32 = 2.5;
pub const RAY_MAX: f32 = 256.0;