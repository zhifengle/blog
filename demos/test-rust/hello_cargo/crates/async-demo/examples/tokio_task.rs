// https://rust-book.junmajinlong.com/ch100/02_understand_tokio_task.html




/*
task 是异步的绿色线程; tokio 管理的线程上下文切换开销比系统管理的线程要小
定义一个 Future, 比如 async { } 就是 Future

只有异步任务才是 tokio task
同步任务推荐放入 blocking thread

task 模块的函数: spawn, spawn_blocking, block_in_place
yield_now(); 放弃 cpu，自己进入队列
unconstrained,spawn_local

*/

#![allow(unused_imports)]
#![allow(dead_code)]

use chrono::Local;

fn now() -> String {
    Local::now().format("%F %T").to_string()
}

fn main() {
    //
}