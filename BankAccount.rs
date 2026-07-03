/*
**Задача 1:** Создайте структуру `BankAccount` , которая моделирует банковский счет. 

Интерфейс структуры должен позволять: 

- Пополнять счет

- Снимать деньги

- Смотреть, сколько на счету денег
*/
use std::io;

struct BankAccount {
    name: String,
    balance: u32,
}

impl BankAccount {
    fn new(name: String) -> Self {
        Self {
            name,
            balance: 0,
        }
    }
}

impl BankAccount {
    fn check_balance(&self) -> u32 {
        self.balance
    }

    fn desopists_balance(&mut self, sum: u32) {
        self.balance += sum;
        println!("Успешно!");
    }

    fn withdraw_balance(&mut self, sum: u32) {
        if self.balance < sum {
            println!("На счету недостаточно средств!");
        } else {
            self.balance -= sum;
            println!("Успешно!");
        }
    }
}

fn main() {
    println!("Приветствую Вас в нашем банке!\nВведи ваше имя: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();

    let mut user = BankAccount::new(name.clone());

    println!("Здравствуйте {name}! Выберите команду: ");
    loop {
        println!("1. Узнать баланс");
        println!("2. Положить деньги");
        println!("3. Снять деньги");
        println!("4. Выход");

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().parse::<u8>().unwrap();

        match command {
            1 => println!("Ваш баланс: {}", user.check_balance()),
            2 => user.desopists_balance(enter_sum()),
            3 => user.withdraw_balance(enter_sum()),
            4 => break,
            _ => println!("Ошибка ввода!")
        }
    }
}

fn enter_sum() -> u32 {
    println!("Введите сумму: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sum = input.trim().parse::<u32>().unwrap();

    sum
}