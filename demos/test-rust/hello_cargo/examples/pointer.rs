#![allow(unused_imports)]
#![allow(dead_code)]

use std::ffi::c_void;

// https://doc.rust-lang.org/std/primitive.pointer.html

// unsafe 里面使用 libc 创建的指针需要手动释放。所以尽量在 Rust 里面创建指针

fn main() {
    // let mut title = [0u8; 10];
    t_box();
}

fn t_box() {
    {
        let a: Box<i32> = Box::new(10);
        // 需要先解引用a
        let b: *const i32 = &*a;
        // 使用 into_raw 来创建
        let c: *const i32 = Box::into_raw(a);
    }
    let c = Box::into_raw(Box::new(10));
    // `{:p}`  打印地址
    println!("{:p}", c);
}
// b'\0' 就是 0
fn truncate_to_first_null_char(input: &mut String) {
    if let Some(index) = input.find('\0') {
        input.truncate(index);
    }
}

fn t_raw_pointer() {
    // 📌 创建裸指针的方式一
    let my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;
    let mut my_speed: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed;
    // 📌 使用 Box::into_raw(my_speed); 需要手动销毁
    let my_speed: Box<i32> = Box::new(88);
    let my_speed: *mut i32 = Box::into_raw(my_speed);

    // By taking ownership of the original `Box<T>` though
    // we are obligated to put it together later to be destroyed.
    unsafe {
        drop(Box::from_raw(my_speed));
    }

    let addr: u32 = 0x41249E;
    let paddr: *const c_void = addr as *const c_void;
    // 替代 DWORD; 这个方式一般在 windows-rs 里面会用到。base 作为一个读取地址的指针
    let mut base: u32 = 0;
    let base = &mut base as *mut u32 as *mut c_void;
}
