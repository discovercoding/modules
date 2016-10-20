use shapes::Shape;
use super::super::ModError;

pub struct Triangle {
    base: f64,
    height: f64
}

impl Triangle {
    pub fn new(base: f64, height: f64) -> Result<Triangle, ModError> {
        if base <= 0.0 {
            Err(ModError::InvalidBase(base))
        } else if height <= 0.0 {
            Err(ModError::InvalidHeight(height))
        } else {
            Ok(Triangle { base: base, height: height })
        }
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}