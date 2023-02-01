use std::{thread, time::Duration};


pub fn run() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Hello from spawned thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Hello from spawned thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}