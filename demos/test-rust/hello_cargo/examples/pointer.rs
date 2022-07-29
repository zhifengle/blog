#![allow(unused_imports)]
#![allow(dead_code)]

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
