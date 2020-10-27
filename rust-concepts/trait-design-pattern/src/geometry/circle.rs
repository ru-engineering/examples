// circle.rs
//use crate::geometry::traits::{Area, Perimeter};  //absolute path
use super::traits::{Area, Perimeter}; //relative to parent

pub struct Circle {
    pub radius: f32,
}

impl Area for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }
}

impl Perimeter for Circle {
    fn perimeter(&self) -> f32 {
        2.0 * 3.14 * self.radius
    }
}
