/*
Задача 2: Прямоугольник Создайте структуру Rectangle и реализуйте методы для вычисления площади, 
периметра и проверки, является ли прямоугольник квадратом
*/

use std::io;

struct Rectangle {
    a: u32,
    b: u32,
}

impl Rectangle {
    fn new(a: u32, b: u32) -> Self {
        Self {
            a,
            b,
        }
    }
}

impl Rectangle {
    fn ploshad(&self) -> u32 {
        self.a * self.b
    }

    fn perim(&self) -> u32 {
        self.a * 2 + self.b * 2
    }
}

impl Rectangle {
    fn check_sqeare(&self) -> bool {
        self.a == self.b
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u32>().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b = input.trim().parse::<u32>().unwrap();

    let rectangle = Rectangle::new(a, b);

    println!("Площадь: {}", rectangle.ploshad());
    println!("Периметр: {}", rectangle.perim());
    println!("Является ли прямоугольник квадратом: {}", rectangle.check_sqeare());
}