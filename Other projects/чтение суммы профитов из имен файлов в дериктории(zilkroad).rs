use std::{io,fs, println, eprintln};
   
fn main() -> io::Result<()> {
    let directoty = fs::read_dir("C:/Users/Skvoc/Downloads/success sales")?;
    let mut total = 0.;

    for files in directoty {
        let file = files?;

        match file.file_name().into_string() {
            Err(e) => {
                eprintln!("Ошибка чтения файла {:?}. Ошибка: {:?}", file.file_name(), e);
                continue;
            },
            Ok(f) => {
                match f
                    .split_ascii_whitespace()
                    .skip(1)
                    .take(1)
                    .map(|s| s[1..].to_string().replace(",", "."))
                    .collect::<String>()
                    .parse::<f64>()
                {
                    Err(e) => {eprintln!("Ошибка чтения профита у файла {:?}. Ошибка: {:?}", f, e);},
                    Ok(s) => {total += s},
                }
            },
        }
    }
    
    println!("Суммарный профит: {:.6} ZEC", total);

    println!("Нажмите Enter для выхода...");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    Ok(())
}