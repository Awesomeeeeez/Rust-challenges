#![allow(dead_code, non_snake_case, unused_variables, unused_assignments, unused_mut, unused_parens, unused_imports)]
mod node;
mod linkedlist;
mod solution;
use linkedlist::{LinkedList, print_list};
//use solution::odd_even_list;
use std::println;

fn main() {
    let mut list1 = LinkedList::new();
    list1.addLast(1);
    list1.addLast(2);
    list1.addLast(3);
    list1.addLast(4);
    list1.addLast(5);
}