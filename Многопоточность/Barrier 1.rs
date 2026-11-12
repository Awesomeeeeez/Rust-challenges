use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let b = Arc::new(Barrier::new(3));
    let mut hs = vec![];

    for i in 0..3 {
        let b = Arc::clone(&b);
        let h = thread::spawn(move || {
            println!("Поток {} готовится...", i);
            thread::sleep(std::time::Duration::from_millis(i as u64 * 100));

            println!("Поток {} ждёт на барьере", i);
            b.wait();

            println!("Поток {} продолжает работу!", i);
        });
        hs.push(h);
    }

    for h in hs {
        h.join().unwrap();
    }
}
