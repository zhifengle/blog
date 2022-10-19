#![allow(unused_imports)]
#![allow(dead_code)]

use std::sync::mpsc;
use std::thread;

// https://doc.rust-lang.org/book/ch16-00-concurrency.html


// https://doc.rust-lang.org/book/ch16-02-message-passing.html
#[test]
fn t_msg() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // 所有权已经转移
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
