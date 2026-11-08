use std::println;

fn main() {
    let num = 123;
    println!("{}", number_of_steps(num));
}

pub fn number_of_steps(num: i32) -> i32 {
    helper(num, 0)
}

fn helper(num: i32, count: i32) -> i32 {
    if num == 0 {
        return count;
    }
    if num % 2 == 0 {
        helper(num / 2, count + 1)
    } else {
        helper(num - 1, count + 1)
    }
}