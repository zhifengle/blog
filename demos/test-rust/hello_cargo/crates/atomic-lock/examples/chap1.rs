#![allow(unused_imports)]
#![allow(dead_code)]

use std::{thread, time::Duration, sync::{Arc, Mutex}};

static X: [i32; 3] = [1, 2, 3];

fn main() {
    mutex_t();
}

fn spawn_t() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
    // ------------------
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
    // ---------------
    let a = Arc::new([1,2,3]);
    let b = a.clone();
    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));

    thread::sleep(Duration::from_secs(5));
}

fn mutex_t() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard);
                // 丢弃了 guard 后，sleep 是并发的
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000);
}
