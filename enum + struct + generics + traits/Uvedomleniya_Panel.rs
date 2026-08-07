/*
Задача 7: Система уведомлений Создайте структуры для разных типов уведомлений 
(Email, SMS, Push) и общий интерфейс для их отправки. 
*/

use std::io;

#[derive(Debug)]
struct Email {
    email: String,
    theme: String,
    text: String,
}

impl Email {
    fn new(email: String, theme: String, text: String) -> Self {
        Self {
            email,
            theme,
            text,
        }
    }

    fn email(&mut self) -> &mut Self {
        println!("Введите email получателя:");
        self.email = vvod_string();
        self
    }

    fn theme(&mut self) -> &mut Self {
        println!("Введите тему письма:");
        self.theme = vvod_string();
        self
    }

    fn text(&mut self) -> &mut Self {
        println!("Введите текст письма:");
        self.text = vvod_string();
        self
    }
}

#[derive(Debug)]
struct SMS {
    number: String,
    text: String,
}

impl SMS {
    fn new(number: String, text: String) -> Self {
        Self {
            number,
            text,
        }
    }

    fn number(&mut self) -> &mut Self {
        println!("Введите номер получателя: ");
        self.number = vvod_string();
        self
    }

    fn text(&mut self) -> &mut Self {
        println!("Введите текст для получателя: ");
        self.text = vvod_string();
        self
    }
}

#[derive(Debug)]
struct Push {
    name_app: String,
    text: String,
}

impl Push {
    fn new(name_app: String, text: String) -> Self {
        Self {
            name_app,
            text,
        }
    }

    fn name_app(&mut self) -> &mut Self {
        println!("Введите название приложения: ");
        self.name_app = vvod_string();
        self
    }

    fn text(&mut self) -> &mut Self {
        println!("Введите текст сообщения: ");
        self.text = vvod_string();
        self
    }
}

fn main() {
    println!("Добро пожаловать в интерфейс отправки уведомлений!");

    loop {
        println!("Выбирите что хотите отправить: ");
        println!("1. Email");
        println!("2. SMS");
        println!("3. Push");
        println!("4. Выход");

        let mut mail = Email::new("".to_string(), "".to_string(), "".to_string());
        let mut sms = SMS::new("".to_string(), "".to_string());
        let mut push = Push::new("".to_string(), "".to_string());

        match vvod_string().as_str() {
            "1" => println!("Ваше письмо успешно отправлено!:\n{:#?}", mail.email().theme().text()),
            "2" => println!("Ваше SMS сообщение успешно отправлено!: \n{:#?}", sms.number().text()),
            "3" => println!("Ваше Push уведомление успешно отправлено!: \n{:#?}", push.name_app().text()),
            "4" => break,
            _ => {println!("Введена неверная цифра!"); continue;},
        }
    }
    println!("Всего доброго!");
}

fn vvod_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}