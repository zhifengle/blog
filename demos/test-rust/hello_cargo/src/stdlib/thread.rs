use std::{thread, time};

fn thread_1() {
    let start = time::Instant::now();
    let handler = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause);
    });
    handler.join().unwrap();
    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}

fn thread_2() {
    let start = time::Instant::now();
    let handler1 = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause);
    });
    let handler2 = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause);
    });
    handler1.join().unwrap();
    handler2.join().unwrap();
    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}

fn thread_3() {
    let pause = time::Duration::from_millis(20);
    for n in 1..101 {
        let mut handlers = Vec::with_capacity(n);
        let start = time::Instant::now();

        for _ in 0..n {
            let handle = thread::spawn(move || {
                // thread::sleep(pause);
                // sleep 的另外的写法
                let start = time::Instant::now();
                while start.elapsed() < pause {
                    thread::yield_now();
                }
            });
            handlers.push(handle);
        }
        while let Some(handle) = handlers.pop() {
            handle.join().unwrap();
        }
        let finish = time::Instant::now();

        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
fn main() {
    thread_3();
}
