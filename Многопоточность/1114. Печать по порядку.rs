use std::sync::{Mutex, mpsc::{self, Receiver, Sender}};

struct Foo {
    tx_a: Sender<()>,
    rx_a: Mutex<Receiver<()>>,

    tx_b: Sender<()>,
    rx_b: Mutex<Receiver<()>>,
}

impl Foo {
    fn new() -> Self {
        let (tx_a, rx_a) = mpsc::channel();
        let (tx_b, rx_b) = mpsc::channel();
        
        Self {
            tx_a,
            rx_a: Mutex::new(rx_a),
            tx_b,
            rx_b: Mutex::new(rx_b)
        }
    }

    fn first<F>(&self, print_first: F)
    where
        F: FnOnce(),
    {
        print_first();
        self.tx_a.send(()).unwrap();
    }

    fn second<F>(&self, print_second: F)
    where
        F: FnOnce(),
    {
        self.rx_a.lock().unwrap().recv().unwrap();
        print_second();
        self.tx_b.send(()).unwrap();
    }

    fn third<F>(&self, print_third: F)
    where
        F: FnOnce(),
    {
        self.rx_b.lock().unwrap().recv().unwrap();
        print_third();
    }
}

use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let foo = Arc::new(Foo::new());

    let f = Arc::clone(&foo);
    let t3 = thread::spawn(move || f.third(|| print!("third")));

    let f = Arc::clone(&foo);
    let t2 = thread::spawn(move || f.second(|| print!("second")));

    let f = Arc::clone(&foo);
    let t1 = thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        f.first(|| print!("first"));
    });

    for t in [t1, t2, t3] { t.join().unwrap(); }
}