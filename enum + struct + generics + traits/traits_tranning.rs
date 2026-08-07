use std::println;



fn main() {
    let mut list = vec![1, 2, 1, 2, 4, 3, 4, 3];
    println!("{:?}", list);
    println!("{:?}", sort(list));
}

fn sort<T>(mut list: Vec<T>) -> Vec<(T, T)> 
where 
    T: Ord + Copy
{
    let mut new_list = Vec::new();
    list.sort();
    for i in (0..list.len()).step_by(2) {
        new_list.push((list[i], list[i+1]))
    }
    new_list
}