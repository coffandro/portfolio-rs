use std::{fmt::{Display, Formatter, Result}, ops::{Add, AddAssign, Mul, Neg, Sub}};

use sdl2::rect::Point;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    #[inline]
    pub fn zero() -> Vector2 {
        return Vector2 { x: 0.0, y: 0.0 }
    }

    #[inline]
    pub fn one() -> Vector2 {
        return Vector2 { x: 1.0, y: 1.0 }
    }

    #[inline]
    pub fn up() -> Vector2 {
        return Vector2 { x: 0.0, y: 1.0 }
    }

    #[inline]
    pub fn down() -> Vector2 {
        return Vector2 { x: 0.0, y: -1.0 }
    }

    #[inline]
    pub fn left() -> Vector2 {
        return Vector2 { x: 1.0, y: 0.0 }
    }

    #[inline]
    pub fn right() -> Vector2 {
        return Vector2 { x: -1.0, y: 0.0 }
    }

    pub fn dot(&mut self, v: Vector2) -> f32 {
        return (self.x * v.x) + (self.y * v.y)
    }

    pub fn length(&mut self) -> f32 {
        return self.dot(*self).sqrt();
    }

    pub fn normalized(&mut self) -> Vector2 {
        let l = self.length();

        return Vector2 {
            x: self.x/l,
            y: self.y/l,
        }
    }

    pub fn normalize(&mut self) {
        let v = self.normalized();

        self.x = v.x;
        self.y = v.y;
    }

    pub fn remove_nan(self) -> Vector2 {
        return Vector2 {
            x: if self.x.is_nan() {0.0} else {self.x},
            y: if self.y.is_nan() {0.0} else {self.y}
        }
    }

    pub fn rotated(self, degree: f32) -> Vector2 {
        return Vector2 {
            x: self.x * f32::cos(degree) - self.y * f32::sin(degree),
            y: self.x * f32::sin(degree) + self.y * f32::cos(degree)
        }
    }

    pub fn rotate(&mut self, degree: f32) {
        let v = self.rotated(degree);

        self.x = v.x;
        self.y = v.y;
    }
}

impl From<Vector2> for Point {
    fn from(value: Vector2) -> Self {
        return Point::new(
            value.x as i32,
            value.y as i32
        )
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        *self = *self + other;
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        let s = self.remove_nan();
        let o= other.remove_nan();

        return Vector2 {
            x: s.x + o.x,
            y: s.y + o.y,
        };
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        return Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: f32) -> Vector2 {
        let s = self.remove_nan();
        let o = if other.is_nan() {0.0} else {other};

        return Vector2 {
            x: s.x * o,
            y: s.y * o
        }
    }
}

impl Mul<i32> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: i32) -> Vector2 {
        let s = self.remove_nan();
        let o = other as f32;

        return Vector2 {
            x: s.x * o,
            y: s.y * o
        }
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        let s = self.remove_nan();
        let o= other.remove_nan();

        return Vector2 {
            x: s.x * o.x,
            y: s.y * o.y
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        return self * -1.0;
    }
}

impl Copy for Vector2 { }

impl Clone for Vector2 {
    fn clone(&self) -> Vector2 {
        *self
    }
}