use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let b = Arc::new(Barrier::new(2));
    let mut hs = vec![];

    for i in 0..2 {
        let b = Arc::clone(&b);
        let h = thread::spawn(move || {
            for round in 1..=3 {
                println!("Поток {}: раунд {}", i, round);
                b.wait();
            }
        });
        hs.push(h);
    }

    for h in hs {
        hs.join().unwrap();
    }
}
