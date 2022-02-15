#![allow(unused_imports)]
#![allow(dead_code)]

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
