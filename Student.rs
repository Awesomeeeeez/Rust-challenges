/*
Задача 3: Система оценок студента Создайте структуру Student с именем и списком оценок. 
Реализуйте методы для добавления оценки и вычисления среднего балла.
*/

use std::io;

struct Student {
    name: String,
    list: Vec<u8>,
}

impl Student {
    fn new(name: String, list: Vec<u8>) -> Self {
        Self {
            name,
            list,
        }
    }
}

impl Student {
    fn average(&self) -> f64 {
        self.list.iter().sum::<u8>() as f64 / self.list.len() as f64
    }

    fn check(&self) {
        println!("Список оценок: {:?}", self.list);
    }

    fn push_num(&mut self, num: u8) {
        self.list.push(num);
        println!("Оценка успешно добавлена!");
    }
}

fn main() {
    let list = Vec::new();
    println!("Введите имя студента: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    println!("Здравствуйте {name}! Введите команду: ");
    let mut student = Student::new(name, list);
    
    loop {
        input.clear();
        println!("1. Добавить оценку");
        println!("2. Просмотреть оценки");
        println!("3. Посчитать средний балл");
        println!("4. Выход");

        io::stdin().read_line(&mut input).unwrap();
        let command: u8 = input.trim().parse::<u8>().unwrap();

        match command {
            1 => student.push_num(vvod_ocenki()),
            2 => student.check(),
            3 => println!("Средний балл: {:.2}", student.average()),
            4 => break,
            _ => println!("Введена не верная команда!")
        }
    }
}

fn vvod_ocenki() -> u8 {
    println!("Введите оценку: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let ocenka = input.trim().parse::<u8>().unwrap();

    ocenka
}