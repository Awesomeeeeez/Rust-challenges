pub struct Stack<T> {
    pub list: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, value: T) -> &mut Self {
        self.list.push(value);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn peek(&self) -> &T {
        &self.list[self.list.len() - 1]
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn get(&self) -> &Vec<T> {
        &self.list
    }
}