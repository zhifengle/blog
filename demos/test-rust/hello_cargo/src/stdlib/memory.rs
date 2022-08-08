#![allow(unused_imports)]
#![allow(dead_code)]

use std::mem::size_of;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

#[test]
fn t_pointer() {
    let a = 42;
    let b = &B;
    let c = &C;

    // b 和 &b[0] 地址是一样的
    println!("{}, {:p}, {:p},{:p}", a, b, &b[0], c);
}

#[test]
fn t_pointer_box() {
    let a: usize = 42;
    let b = &B;
    let c: Box<[u8]> = Box::new(C);

    // b 和 &b[0] 地址是一样的
    println!("{}, {:p}, {:p},{:p}", a, b, &b[0], c);

    // a
    println!("a (an unsigned integer):");
    println!("  location:   {:p}", &a);
    println!("  size:       {:?} bytes", size_of::<usize>());
    println!("  value:      {:?}", a);
    println!();

    // b
    println!("b (a reference to B):");
    println!("  location:   {:p}", &b);
    println!("  size:       {:?} bytes", size_of::<[u8; 10]>());
    println!("  points to:  {:p}", b);
    println!();

    println!("c (a reference to C):");
    println!("  location:   {:p}", &c);
    println!("  size:       {:?} bytes", size_of::<Box<u8>>());
    println!("  points to:  {:p}", c);
    println!();

    println!("B (an array of 11 bytes):");
    println!("  location:   {:p}", &B);
    println!("  size:       {:?} bytes", size_of::<[u8; 10]>());
    println!("  value:      {:?}", B);
    println!();

    println!("C (an array of 11 bytes):");
    println!("  location:   {:p}", &C);
    println!("  size:       {:?} bytes", size_of::<[u8; 11]>());
    println!("  value:      {:?}", C);
    println!();
}

#[test]
fn t_pointer_string() {
    use std::borrow::Cow;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    let a = 42;
    let b: String;
    let c: Cow<str>;
    unsafe {
        // 地址 ----> [u8] ----> buf
        let b_ptr = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        // 地址 ----> [u8] ----> 再转成指针
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("a: {}, b: {}, c: {}", a, b, c);
    std::process::exit(0);
}

#[test]
fn t_raw_pointer() {
    let a: i64 = 64;
    // 强制转换一个指针为 raw pointer
    let a_ptr = &a as *const i64;
    // 指针地址是安全的
    println!("a: {} ({:p})", a, a_ptr);
    // 读 raw pointer 的值是不安全的
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };
    // Raw pointers are often handled in Rust code by the OS or a third-party library.
    println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr + 7);

    let ptr = 42 as *const Vec<String>;
    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}
