use std::format;

pub trait Shape {
    // Определите методы
    fn area(&self) -> f64;

    fn perimeter(&self) -> f64;

    fn describe(&self) -> String {
        format!("Площадь: {}, Периметр: {}", self.area(), self.perimeter())
    }
}