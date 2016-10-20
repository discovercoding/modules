pub mod triangle;
pub mod circle;

pub trait Shape {
    fn area(&self) -> f64;
}
