use std::thread;

pub fn start_n_threads() {
    const N: isize = 10;
    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread {}!", i);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn current_thread() {
    let current_td = thread::current();
    println!(
        "current_thread: {:?}, {:?}",
        current_td.id(),
        current_td.name()
    );

    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);
    let handler = builder.spawn(|| {
        let current_thread = thread::current();
        println!(
            "child thread: {:?}, {:?}",
            current_thread.id(),
            current_thread.name()
        );
    }).unwrap();
    handler.join().unwrap();
}
fn main() {
    // start_n_threads();
    current_thread();
}
