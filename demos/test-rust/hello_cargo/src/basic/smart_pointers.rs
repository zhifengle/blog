// https://doc.rust-lang.org/book/ch15-01-box.html

/*
enum List {
    Cons(i32, List),
    Nil,
}
 */

enum List {
    // 递归的数据。使用 Box，这样编译器能够识别
    // 编译的的时候，大小是不确定的
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

#[test]
fn t_box() {
    // 存储在堆上面的数据
    let b = Box::new(5);
    println!("b = {}", b);
    let _list = Cons(1, Box::new(Cons(3, Box::new(Nil))));
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        Self(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn t_my_box() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // 需要实现 deref
    assert_eq!(5, *y);
}