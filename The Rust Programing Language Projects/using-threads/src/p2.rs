use std::{thread, time::Duration};


pub fn run() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    for i in 0..5 {
        println!("Hello from spawned thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}