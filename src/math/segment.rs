use std::{fmt::{Display, Formatter}, ops::Mul};

use serde::Deserialize;

use crate::math::Vector2;


#[derive(Deserialize, Debug, Clone)]
pub struct Segment {
    pub a: Vector2,
    pub b: Vector2
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

impl Mul<i32> for Segment {
    type Output = Segment;

    fn mul(self, mult: i32) -> Segment {
        return Segment {
            a: self.a * mult,
            b: self.b * mult
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}