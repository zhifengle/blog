#![allow(unused_imports)]
#![allow(dead_code)]

use std::ops::Range;

// Iterator 是一种特征(trait)
trait MyIterator {
    // https://course.rs/basic/trait/advance-trait.html
    // Item 是关联类型
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
fn t_turbofish() {
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
    // https://techblog.tonsser.com/posts/what-is-rusts-turbofish
    let a = [1, 2, 3];
    let doubled = a.iter().map(|x| x * 2).collect::<Vec<i32>>();
    // 这种写法也行
    // let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    assert_eq!(vec![2, 4, 6], doubled);

    // Windows git clone 下来的文件换行符是 \r\n. 替换 \r\n 为 \n
    // let expected: String = expected.lines().collect::<Vec<&str>>().join("\n");
}

// ref: blog/**/cut-rs
fn extract_bytes(line: &str, pos: &[Range<usize>]) -> String {
    let bytes = line.as_bytes();
    let selected: Vec<_> = pos
        .iter()
        .cloned()
        // 需要复制一份
        // filter_map: map ---> filter ---> map; 可以直接看文档示例
        .flat_map(|range| range.filter_map(|i| bytes.get(i)).copied())
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

// ref: blog/**/cut-rs
fn extract_chars2(line: &str, pos: &[Range<usize>]) -> String {
    let chars: Vec<_> = line.chars().collect();
    pos.iter()
        .cloned()
        // flat_map ==  map + flatten
        .flat_map(|range| range.filter_map(|i| chars.get(i)))
        // 展开双重包裹的 Iterator
        // .flatten()
        .collect()
}

#[test]
fn t_extract_bytes() {
    // 这里的 á 分成字节后，解析不出字符
    assert_eq!(extract_bytes("ábc", &[0..1]), "�".to_string());
    assert_eq!(extract_bytes("ábc", &[0..2]), "á".to_string());
    assert_eq!(extract_bytes("ábc", &[0..3]), "áb".to_string());
    assert_eq!(extract_bytes("ábc", &[0..4]), "ábc".to_string());
    assert_eq!(extract_bytes("ábc", &[3..4, 2..3]), "cb".to_string());
    assert_eq!(extract_bytes("ábc", &[0..2, 5..6]), "á".to_string());
}

// https://doc.rust-lang.org/stable/rust-by-example/trait/iter.html
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fib() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

#[test]
fn t_fib() {
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fib().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fib().skip(4).take(4) {
        println!("> {}", i);
    }
}
