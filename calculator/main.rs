// тренировка enum, дженериков, трейтов 

mod calculator;
mod traits;

use std::{io, println};
use calculator::Calculator;

use crate::traits::Calculate;

fn main() {
    println!("Введите первое число: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num1 = input.trim().parse::<f64>().unwrap();

    println!("Введите второе число: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let num2 = input.trim().parse::<f64>().unwrap();

    println!("Выберите операцию: ");
    println!("1. +");
    println!("2. -");
    println!("3. *");
    println!("4. /");

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let operation = input.trim();

    let op = match operation {
        "1" => Calculator::Add(num1, num2),
        "2" => Calculator::Subtract(num1, num2),
        "3" => Calculator::Multiply(num1, num2),
        "4" => Calculator::Divide(num1, num2),
        _ => panic!("Неизвестная операция!"),
    };

    println!("Результат: {:.2}", op.calculate());
}