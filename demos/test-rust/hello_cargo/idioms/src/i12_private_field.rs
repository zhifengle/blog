mod a {
    pub struct S {
        pub foo: i32,
        bar: i32,
    }
}
fn i12_private_field(s: a::S) {
    // 使用  ..  来
    // 使用私有的域，保证未来的兼容性
    let a::S { foo: _, .. } = s;
}
