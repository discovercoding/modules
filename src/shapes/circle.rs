use std::f64::consts::PI;
use shapes::Shape;
use super::super::ModError;

pub struct Circle {
    radius: f64
}

impl Circle {
    pub fn new(radius: f64) -> Result<Circle, ModError> {
        if radius <= 0.0 {
            Err(ModError::InvalidRadius(radius))
        } else {
            Ok(Circle { radius: radius })
        }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}