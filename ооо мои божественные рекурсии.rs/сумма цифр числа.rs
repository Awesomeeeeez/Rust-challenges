use std::println;

fn main() {
    println!("{}", fib(534));
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else {
        n % 10 + fib(n / 10)
    }
}