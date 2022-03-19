//! https://rust-unofficial.github.io/patterns/intro.html
#![allow(dead_code)]

mod i04_the_default_trait;
mod i06_destructor;
mod i07_mem_take_enums;
mod i08_ffi;
mod i10_iter_option;
mod i11_pass_var_closure;
mod i12_private_field;
mod i14_temporary_mut;
mod use_borrowed_types;

fn main() {
    say_hello("John");
}

// 使用 format! 来拼接字符串
fn say_hello(name: &str) -> String {
    format!("Hello {}!", name)
}

// rust 没有构造函数，使用静态方法 new

// 使用 Deref ??  没看懂 ; 2.5

// 2.7  使用 mem::take(xxx),  replace 来改变 enum

// 2.8 没太明白。 动态分配赋值 ?   readable:  io, 或者 文件
// 2.9 没太明白?? 跳过

// 2.13 文档相关 跳过
