use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct TaskQueue {
    queue: Mutex<VecDeque<String>>,
    cvar: Condvar,
}

impl TaskQueue {
    fn new() -> Self {
        TaskQueue {
            queue: Mutex::new(VecDeque::new()),
            cvar: Condvar::new(),
        }
    }

    fn push(&self, task: String) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(task);
        self.cvar.notify_one();
    }

    fn pop(&self) -> String {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.cvar.wait(queue).unwrap();
        }
        queue.pop_front().unwrap()
    }
}

fn main() {
    let queue = Arc::new(TaskQueue::new());

    let queue_clone = Arc::clone(&queue);
    let worker = thread::spawn(move || {
        for _ in 0..3 {
            let task = queue_clone.pop();
            println!("Выполняю: {}", task);
        }
    });

    for i in 1..=3 {
        thread::sleep(std::time::Duration::from_millis(500));
        queue.push(format!("Задача {}", i));
        println!("Добавлена задача {}", i);
    }

    worker.join().unwrap();
}
