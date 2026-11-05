use std::println;

fn main() {
    println!("{}", fib(3, 3));
}

fn fib(a: i32, n: i32) -> i32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return a;
    } else {
        return a * fib(a, n-1);
    }
}