/*
Задача 4: Tuple struct для координат Создайте tuple struct Point3D для 
трехмерных координат и реализуйте метод вычисления расстояния до начала координат.
*/

struct Point3D(f64, f64, f64);

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}

impl Point3D {
    fn distance_to_origin(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}

fn main() {
    let point = Point3D::new(2.0, 3.0, 5.0);

    println!("{:.2}", point.distance_to_origin());
}
