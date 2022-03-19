use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            // 可以使用 clone，只是个反面模式
            name: mem::take(name),
        }
    }
}

// match 的例子省略了
