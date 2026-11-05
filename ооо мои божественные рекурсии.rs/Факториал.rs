use std::println;

fn main() {
    println!("{}", fib(5));
}

fn fib(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else {
        n * fib(n-1)
    }
}