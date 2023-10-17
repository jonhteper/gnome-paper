use chrono::Local;
use std::io;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

mod domain;

fn task() {
    let now = Local::now();
    println!("async task running at {:?}", now);
}

fn background_task(stop_signal: Arc<AtomicBool>) {
    task();
    while !stop_signal.load(Ordering::Relaxed) {
        thread::sleep(Duration::from_millis(5000));
        task();
    }
}

pub fn main() {
    let stop_signal = Arc::new(AtomicBool::new(false));
    let signal_clone = Arc::clone(&stop_signal);
    let handle = thread::spawn(move || background_task(signal_clone));

    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading input");

        match buffer.as_str().trim() {
            "close" => {
                stop_signal.store(true, Ordering::Relaxed);
                break;
            }
            _ => println!("el usuario dice: {}", &buffer),
        }
    }

    handle.join().unwrap();
}
