use std::{print, sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread};

struct ZeroEvenOdd {
    n: i32,
    tx_0: Sender<()>,
    rx_0: Mutex<Receiver<()>>,
    tx_1: Sender<()>,
    rx_1: Mutex<Receiver<()>>,
    tx_2: Sender<()>,
    rx_2: Mutex<Receiver<()>>,
}

impl ZeroEvenOdd {
    fn new(n: i32) -> Self {
        let (tx_0, rx_0) = mpsc::channel();
        let (tx_1, rx_1) = mpsc::channel();
        let (tx_2, rx_2) = mpsc::channel();

        Self {
            n,
            tx_0,
            rx_0: Mutex::new(rx_0),
            tx_1,
            rx_1: Mutex::new(rx_1),
            tx_2,
            rx_2: Mutex::new(rx_2),
        }
    }

    fn zero<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for i in 0..self.n {
            print_number(0);

            if i % 2 == 0 {
                self.tx_1.send(()).unwrap();
            } else {
                self.tx_0.send(()).unwrap();
            }

            self.rx_2.lock().unwrap().recv().unwrap();
        }
    }

    fn even<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for i in (2..=self.n).step_by(2) {
            self.rx_0.lock().unwrap().recv().unwrap();
            print_number(i);
            self.tx_2.send(()).unwrap();
        }
    }

    fn odd<F>(&self, print_number: F)
    where
        F: Fn(i32),
    {
        for i in (1..=self.n).step_by(2) {
            self.rx_1.lock().unwrap().recv().unwrap();
            print_number(i);
            self.tx_2.send(()).unwrap();
        }
    }
}

fn main() {
    let a = Arc::new(ZeroEvenOdd::new(1));
    let mut threads = Vec::new();

    let clone = Arc::clone(&a);
    let t = thread::spawn(move || clone.zero(|x| print!("{x}")));
    threads.push(t);

    let clone = Arc::clone(&a);
    let t = thread::spawn(move || clone.even(|x| print!("{x}")));
    threads.push(t);

    let clone = Arc::clone(&a);
    let t = thread::spawn(move || clone.odd(|x| print!("{x}")));
    threads.push(t);

    for i in threads {
        i.join().unwrap()
    }
}