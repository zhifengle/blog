#![allow(unused_imports)]
#![allow(dead_code)]

// https://tokio.rs/tokio/tutorial/hello-tokio
// https://rust-book.junmajinlong.com/ch100/01_understand_tokio_runtime.html
// 这里其实就是 tokio 文档的翻译
// https://course.rs/async-rust/tokio/spawning.html

use std::{thread, time::Duration};

use chrono::Local;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    // block_on 参数是 future
    // async { } 直接定义了一个 future
    rt.block_on(async {
        println!("before sleep: {}", Local::now().format("%F %T.%3f"));
        // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        // op 这个 future 定义了。还没有执行
        let op = tokio::time::sleep(tokio::time::Duration::from_secs(2));
        // 执行
        op.await;
        println!("after sleep: {}", Local::now().format("%F %T.%3f"));
    });
    enter_task();
    // block_on 会阻塞线程；这个例子是阻塞 main 的主线程
    // 最后打印就能看出来
    println!("main finish");
}
// ===================================

fn t_main() {
    // 这是宏 tokio::main 转换后的接口
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}

async fn say_world() {
    println!("world");
}

// #[tokio::main]

// 单一线程
// #[tokio::main(flavor = "current_thread")]
async fn t_tokio_main() {
    let op = say_world();
    println!("hello");
    // 调用 .await 后才真正的执行
    op.await;
}

// ===================================

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn enter_task() {
    let rt = Runtime::new().unwrap();
    // 进入但是不阻塞; 声明后，接下来的任务都在 runtime 的上下文执行
    let guard1 = rt.enter();
    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!("task1 is over {}", now());
    });
    drop(guard1);
    let guard2 = rt.enter();
    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
        println!("task2 is over {}", now());
    });
    drop(guard2);
    // 避免结束退出了。让上面的线程任务都执行完
    thread::sleep(std::time::Duration::from_secs(10));
}

// =================================

fn two_runtime() {
    // JoinHandle
    let t1 = thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        println!("start t1");
        thread::sleep(Duration::from_secs(10));
        println!("end t1");
    });
    let t2 = thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        println!("start t2");
        thread::sleep(Duration::from_secs(10));
        println!("end t2");
    });
    t1.join().unwrap();
    t2.join().unwrap();
}
