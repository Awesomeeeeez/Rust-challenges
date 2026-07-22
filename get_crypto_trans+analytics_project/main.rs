use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader, Write}};

// use rand = "0.8" in dependencies
use rand::Rng;

fn main() {
    let path = "test.txt";
    try_file(path);
    file_check(path);
}

fn try_file(path: &str) {
    match File::create(path) {
        Ok(file) => gen_trans(file),
        Err(_) => println!("Ошибка создания файла!")
    }
}

fn gen_trans(mut file: File) {
    let trs_type = vec!["Deposit", "Withdraw"];
    let type_currency = vec!["USDT", "TON", "SOL", "ETH", "USDC"];

    for _ in 1..=input_check() {
        let line = format!(
            "{} {} {}\n", 
            trs_type[rand::thread_rng().gen_range(0..=1)],
            rand::thread_rng().gen_range(-1000..1000),
            type_currency[rand::thread_rng().gen_range(0..=4)]
        );

        if let Err(_) = file.write_all(&line.as_bytes()) {
            println!("Ошибка записи файла!");
        }
    }
}

fn input_check() -> u32 {
    let count = 
        loop {
            print!("Введите количество транзакций, которое вы хотите сгенерировать: ");
            io::stdout().flush().unwrap(); // не знаю что это, но без этого не будет корректно работать print! :)

            let mut input = String::new();
            
            if let Err(_) = io::stdin().read_line(&mut input) {
                println!("Ошибка ввода, попробуйте снова!");
                continue;
            }

            match input.trim().parse::<u32>() {
                Ok(v) if v > 0 => break v,
                Ok(_) => println!("Число не может ровняться 0!"),
                Err(_) => println!("Число должно быть больше нуля и целым!")
            }
        };
    return count;
}

fn file_check(path: &str) {
    let file = 
        match File::open(path) {
            Ok(v) => v,
            Err(e) => {
                println!("Не удалось открыть файл: {e}");
                return;
            }
        };
    let reader = BufReader::new(file); // без понятия что такое BufReader

    let mut count_deposits = 0;
    let mut count_withdraws = 0;
    let mut list_trs_in = HashMap::new();
    let mut list_trs_out = HashMap::new();
    let mut list_sum = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(v) => {
                
                let vec_line: Vec<String> = v.split_ascii_whitespace().map(|s|s.to_string()).collect();

                if v.contains("Deposit") {
                    count_deposits += 1;
                    *list_trs_in.entry(vec_line[2].clone()).or_insert(0) += 1;
                } else {
                    count_withdraws += 1;
                    *list_trs_out.entry(vec_line[2].clone()).or_insert(0) += 1;
                }

                *list_sum.entry(vec_line[2].clone()).or_insert(0) += vec_line[1].parse::<i32>().unwrap();
            }
            Err(e) => println!("Ошибка чтения строки: {}", e)
        }
    }

    println!("Всего депозитов: {}", count_deposits);
    println!("Всего выводов: {}", count_withdraws);

    for (k, v) in &list_sum {
        println!("{k}: Всего депозитов: {}, Всего выводов: {}, Остаток: {v}", 
            match list_trs_in.get(k) {
                Some(v) => v.to_string(),
                None => "Депозитов не было".to_string()
            },
            
            match list_trs_out.get(k) {
                Some(v) => v.to_string(),
                None => "Выводов не было".to_string()
            });
    }
}