// https://course.rs/async-rust/tokio/shared-state.html

#![allow(unused_imports)]
#![allow(dead_code)]

use std::{sync::{Arc, Mutex}, collections::HashMap};

use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};

// bytes 克隆操作效率更高
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("listening");
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    todo!()
}

// 使用 tokio 的异步锁
async fn inc_and_do_stuff(mutex: &tokio::sync::Mutex<i32>) {
    let mut lock = mutex.lock().await;
    *lock += 1;
    // do_sth_async().await;
}