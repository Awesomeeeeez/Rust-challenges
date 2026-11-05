use std::println;

fn main() {
    println!("{}", fib("123321".to_string()));
}

fn fib(a: String) -> bool {
    if a.len() == 1 || a.len() == 0 {
        return true;
    }
    if a[0..1] != a[a.len()-1..] {
        return false;
    }
    else {
        fib(a[1..a.len()-1].to_string())
    }
}