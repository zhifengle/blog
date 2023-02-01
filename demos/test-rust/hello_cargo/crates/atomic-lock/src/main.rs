use std::thread;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    println!("main thread");
    t1.join().unwrap();
    t2.join().unwrap();
    let nums = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = nums.len();
        let sum = nums.into_iter().sum::<usize>();
        sum / len
    });
    let ave = t.join().unwrap();
    println!("ave is: {ave}");
}

fn f() {
    println!("thread!");
    let id = thread::current().id();
    println!("current thread id: {id:?}");
}