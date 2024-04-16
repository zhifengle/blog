mod basic;
mod data;
mod misc;
mod stdlib;

// 📌 默认都是已经验证了的 demo。不再放入 mod.rs 。这样能减少不必要的载入

// 📌 basic 里面是一些基础的代码。来自官方 book。
// https://doc.rust-lang.org/book/
// https://doc.rust-lang.org/stable/rust-by-example/index.html

// 📌 data 里面是一些数据类型的 demo。

// 📌 stdlib 一些标准库的用法
// 部分来自 https://rust-lang-nursery.github.io/rust-cookbook/about.html

// https://programming-idioms.org/

// misc 随便放的; 有的依赖非标准库
fn main() {
    // cargo run 11 poem.txt
    // minigrep::mingrep_main();
}
