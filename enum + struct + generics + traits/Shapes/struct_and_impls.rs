use crate::traits::Shape;

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powf(2.)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }

    fn perimeter(&self) -> f64 {
        self.height * 2. + self.width * 2.
    }
}

pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let p_p = self.perimeter() / 2.;
        (p_p * (p_p - self.a) * (p_p - self.b) * (p_p - self.c)).sqrt()
    }

    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

const PI: f64 = 3.14;