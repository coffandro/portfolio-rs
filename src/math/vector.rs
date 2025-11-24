use std::{fmt::{Display, Formatter, Result}, ops::{Add, AddAssign, Mul, Neg, Sub}};

use sdl2::rect::Point;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

#[derive(Deserialize, Debug, Clone)]
pub struct Segment {
    pub a: Vector2,
    pub b: Vector2
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

    pub fn to_point(self) -> Point {
        return Point::new(
            self.x as i32,
            self.y as i32
        )
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
}

impl Segment {
    pub fn distance_to_point(self: &Segment, point: Vector2) -> f32 {
        let a = self.a;
        let b = self.b;
        let e = point;
        
        // vector AB
        let ab = Vector2 {
            x: b.x-a.x,
            y: b.y-a.y
        };

        // vector BP
        let be = Vector2 {
            x: e.x - b.x,
            y: e.y - b.y
        };

        // vector AP
        let ae = Vector2 {
            x: e.x - a.x,
            y: e.y - a.y
        };

        // Calculating the dot product
        let ab_be = ab.x * be.x + ab.y * be.y;
        let ab_ae = ab.x * ae.x + ab.y * ae.y;

        if ab_be > 0.0 {
            let x = e.x - b.x;
            let y = e.y - b.y;
            return f32::sqrt(x*x + y*y);
        } else if ab_ae < 0.0 {
            let x = e.x - a.x;
            let y = e.y - a.y;
            return f32::sqrt(x*x + y*y);
        } else {
            let x1 = ab.x;
            let y1 = ab.y;
            let x2 = ae.x;
            let y2 = ae.y;
            let m = f32::sqrt(x1*x1 + y1*y1);
            return f32::abs(x1*y2 - y2*x2) / m;
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.a, self.b)
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

impl Mul<i32> for Segment {
    type Output = Segment;

    fn mul(self, mult: i32) -> Segment {
        return Segment {
            a: self.a * mult,
            b: self.b * mult
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