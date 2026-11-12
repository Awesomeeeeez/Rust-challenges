use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let p = Arc::new((Mutex::new(false), Condvar::new()));
    let pc = Arc::clone(&p);

    let w = thread::spawn(move || {
        let (lock, cvar) = &*pc;

        let mut r = lock.lock().unwrap();
        while !*r {
            println!("Ожидаем сигнал...");
            ready = cvar.wait(r).unwrap();
        }
        println!("Получили сигнал, продолжаем!");
    });

    thread::sleep(std::time::Duration::from_secs(1));

    let (lock, cvar) = &*p;
    {
        let mut r = lock.lock().unwrap();
        *r = true;
        println!("Отправляем сигнал!");
    }
    cvar.notify_one();

    w.join().unwrap();
}