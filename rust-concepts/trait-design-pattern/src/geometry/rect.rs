// rect.rs
use crate::geometry::traits::{Area, Perimeter};

pub struct Rect {
    pub width: f32,
    pub length: f32,
}

impl Area for Rect {
    fn area(&self) -> f32 {
        self.width * self.length
    }
}

impl Perimeter for Rect {
    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.length)
    }
}
