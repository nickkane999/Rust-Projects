use std::sync::mpsc;
use std::{thread, time::Duration};

pub fn run() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for v in val {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}