mod traits;
mod struct_and_impls;

use std::println;

use struct_and_impls::{Circle, Rectangle, Triangle};

use crate::traits::Shape;

fn main() {
    let circle = Circle {radius: 0.0};
    let rectangle = Rectangle {width: 2.0, height: 5.0};
    let triangle = Triangle {a: 0.0, b: 0.0, c: 0.0};

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rectangle), Box::new(triangle)];

    for (i, shape) in shapes.iter().enumerate() {
        println!("{i}. Площадь: {}", shape.area());
        println!("{i}. Периметр: {}", shape.perimeter());
        println!("{i}. Описание: {}", shape.describe());
    }
}