// https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
#![allow(unused_imports)]
#![allow(dead_code)]


use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

struct Song;

async fn learn_song() -> Song {
    println!("learn song");
    Song
}
async fn sing_song(song: Song) {
    println!("sing song");
}
async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // 同时执行多个
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
