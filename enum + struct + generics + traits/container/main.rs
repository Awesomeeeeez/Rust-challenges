mod container;

use std::println;
use container::Container;

fn main() {
    let content = vec![1, 2, 3];
    let mut container = Container::new(45, content);

    println!("Содержимое контейнера: {:?}", container.get());
    container.clear();
    println!("Содержимое контейнера: {:?}", container.get());
    println!("Пуст ли контейнер: {}", container.is_empty());
}
