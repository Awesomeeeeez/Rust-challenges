// практика enum и traits

mod traffic_light;
mod traits;

use std::{io, print, println};
use crate::traits::Next;
use traffic_light::TrafficLight;

fn main() {
    loop {
        println!("Выберите начальный цвет светофора: ");
        println!("1. Red");
        println!("2. Yellow");
        println!("3. Green");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num = input.trim().parse::<u8>().unwrap();

        if num > 3 || num < 1 {println!("Ошибка ввода! Повторите попытку"); continue;}

        let traffic_light = match num {
            1 => TrafficLight::Red,
            2 => TrafficLight::Yellow,
            3 => TrafficLight::Green,
            _ => TrafficLight::Red,
        };

        print!("Следующий сигнал светофора: ");
        println!("{:?}", traffic_light.next());
        break;
    }
}