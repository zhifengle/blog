#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {
    pub fn new() -> Self {
        FooBuilder {
            bar: String::from("X"),
        }
    }
    // 有副作用的修改
    pub fn name(mut self, bar: String) -> Self {
        self.bar = bar;
        self
    }
    // 把之前的修改全部应用到 Foo
    pub fn build(self) -> Foo {
        Foo { bar: self.bar }
    }
}
