use std::println;

fn find_max<T>(items: &[T]) -> Option<T>
where
    T: PartialOrd + Clone
{
    // Ваша реализация
    let mut max: Option<T> = None;

    for i in items {
        if max.is_none() {
            max = Some(i.clone());
        } else {
            if max.as_ref().unwrap() < i {
                max = Some(i.clone());
            }
        }
    }
    max
}

// Тесты
fn main() {
    let numbers = vec![1, 5, 3, 9, 2];
    println!("{:?}", find_max(&numbers));

    let strings = vec!["apple", "zebra", "banana"];
    println!("{:?}", find_max(&strings));
}