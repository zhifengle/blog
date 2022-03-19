fn bar() -> Result<(), ()> {
    // 不一定要在函数里面定义
    struct Foo;

    // 缺点是不一定保证运行，比如死循环
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }
    let _exit = Foo;
    // baz()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        bar();
    }
}
