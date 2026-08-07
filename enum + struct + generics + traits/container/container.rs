use std::println;

pub struct Container<T> {
    id: u8,
    content: Vec<T>,
}

impl<T> Container<T> {
    pub fn new(id: u8, content: Vec<T>) -> Self {
        println!("Контейнер создан!");
        Self {
            id,
            content,
        }
    }

    pub fn get(&self) -> &Vec<T> {
        &self.content
    }

    pub fn clear(&mut self) {
        self.content.clear();
        println!("Контейнер очищен!");
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}