use std::println;

fn main() {
    println!("{}", fib(451, 287));
}

pub fn fib(a: i32, b: i32) -> i32 {
    if a.max(b) % a.min(b) == 0 {
        return a.min(b);
    }
    else {
        fib(a.max(b) % a.min(b), a.min(b))
    }
}