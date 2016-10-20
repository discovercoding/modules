extern crate modules;
use modules::Shape;

fn display_area<S: Shape>(shape: &S) {
    println!("The area is {}", shape.area());
}

fn main() {
    let t = modules::Triangle::new(1.5, 2.3).unwrap();
    let c = modules::Circle::new(1.2).unwrap();
    display_area(&t);
    display_area(&c);

    let c = modules::Color::new(12, 13, 34).unwrap();
    println!("The color is {}", &c);
}
