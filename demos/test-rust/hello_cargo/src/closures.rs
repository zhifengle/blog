// https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html

fn apply<F>(f: F)
where
    F: FnOnce(),
    // F: FnMut(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

#[test]
fn t_closure_as_parameters() {
    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    // 捕获了 farewell, 并丢弃; 所以是 FnOnce
    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("farewell {}", farewell);

        mem::drop(farewell);
    };
    apply(diary);

    // apply_to_3 返回的 i32，所以这里类型被自动推导出来
    let double = |x| 2 * x;
    println!("apply to 3: {}", apply_to_3(double));
}
