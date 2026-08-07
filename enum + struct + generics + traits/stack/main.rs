mod stack;

use std::println;
use stack::Stack;

fn main() {
    let mut list = Vec::new();
    let mut stack = Stack {list};

    stack.push(1).push(2).push(3);

    println!("Просмотр стека: {:?}", stack.get());
    println!("Длина стека: {}", stack.len());
    println!("Последний элемент стека: {}", stack.peek());
    println!("Удаление и возврат последнего элемента из стека: {:?}", stack.pop());
    println!("Просмотр стека: {:?}", stack.get());
    println!("Длина стека: {}", stack.len());
}
