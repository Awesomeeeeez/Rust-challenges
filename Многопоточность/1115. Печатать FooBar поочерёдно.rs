use std::{print, sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread};

struct FooBar {
    n: usize,
    tx: Sender<()>,
    rx: Mutex<Receiver<()>>,
    tx_a: Sender<()>,
    rx_a: Mutex<Receiver<()>>,
}

impl FooBar {
    fn new(n: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let (tx_a, rx_a) = mpsc::channel();
        FooBar {
            n,
            tx,
            rx: Mutex::new(rx),
            tx_a,
            rx_a: Mutex::new(rx_a),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            print_foo();
            self.tx.send(()).unwrap();
            self.rx_a.lock().unwrap().recv().unwrap();
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            self.rx.lock().unwrap().recv().unwrap();
            print_bar();
            self.tx_a.send(()).unwrap();
        }
    }
}

fn main() {
    let a   = Arc::new(FooBar::new(15));
    let mut threads = Vec::new();

    let f = Arc::clone(&a);
    let t1 = thread::spawn(move || f.foo(|| print!("foo")));
    threads.push(t1);

    let f = Arc::clone(&a);
    let t2 = thread::spawn(move || f.bar(|| print!("bar")));
    threads.push(t2);

    for i in threads {
        i.join().unwrap()
    }
}