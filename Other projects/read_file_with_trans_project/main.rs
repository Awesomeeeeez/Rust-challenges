use std::{fs::File, io::Read};

struct Transaction {
    id: u32,
    tx_type: String,
    amount: f64,
    currency: String
}

fn try_open_file() -> Result<String, std::io::Error> {
    let mut content = String::new();
    let mut file = File::open(r"C:\Users\Skvoc\rust-demo\src\transactions.csv")?; // установите свой путь
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let mut transactions = Vec::new();
    let mut log_vec = Vec::new();
    let mut total_deposits = 0.;
    let mut total_withdrawals = 0.;

    let content = 
        match try_open_file() {
            Ok(c) => c,
            Err(e) => {
                        println!("{}", e);
                        return;
                    }
        };

    for line in content.split("\n") {
        if line.starts_with("id,") {
            continue;
        }

        let trans: Vec<&str> = line.split(",").collect();
        if trans.len() != 4 {
            log_vec.push(trans);
            continue;
        }

        let tx = Transaction {
            id:
                match trans[0].parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => {
                        log_vec.push(trans);
                        continue;
                    }
                },
            tx_type: trans[1].to_string(),
            amount:
                    match trans[2].parse::<f64>() {
                        Ok(v) => v,
                        Err(_) => {
                                log_vec.push(trans);
                                continue;
                            }
                    },
            currency: trans[3].to_string()
        };

        match tx.tx_type.as_str() {
            "deposit" => total_deposits += tx.amount,
            "withdraw" => total_withdrawals += tx.amount,
            _ => {
                log_vec.push(trans);
                continue;
            }
        }
        transactions.push(tx);
    }
    calculate_stats(log_vec, transactions, total_deposits, total_withdrawals);
}

fn calculate_stats(log_vec: Vec<Vec<&str>>, transactions: Vec<Transaction>, total_deposits: f64, total_withdrawals: f64) {
    println!("Всего траназкций: {}", log_vec.len() + transactions.len());
    println!("Успешных транзакций: {}", transactions.len());
    println!("Неуспешных транзакций: {}", log_vec.len());
    println!("Сумма пополнений: {}", total_deposits);
    println!("Сумма вывода: {}", total_withdrawals);
    println!("Баланс: {}", total_deposits - total_withdrawals);

    if !log_vec.is_empty() {
        println!("Проблемные транзакции: ");
        for i in &log_vec {
            println!("{:?}", i);
        }
    }
}