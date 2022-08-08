use std::{
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

// ref: ct 类型系统
// https://github.com/tyrchen/geektime-rust/tree/master/23_advanced_generics
// 泛型参数做延时绑定，PhantomData 提供额外的类型

// chap24: trait object 额外的内存访问 . 30%;  分配内存会多20倍

#[derive(Debug, Default, PartialEq, Eq)]
struct Identifier<T> {
    inner: u64,
    // 定义的时候用不上，后面用得上
    // 只是用来标记
    _tag: PhantomData<T>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct User {
    // 这样 user.id 和 product.id 就不一样了
    id: Identifier<Self>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Product {
    id: Identifier<Self>,
}

#[test]
fn t_not_same_id() {
    let user = User::default();
    let product = Product::default();

    // 下面一行报错，两者不能比较
    // assert_ne!(user.id, product.id);
    assert_eq!(user.id.inner, product.id.inner);
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

struct Customer<T> {
    id: u64,
    name: String,
    // 没有这行，泛型定义会失败
    _type: PhantomData<T>,
}

trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

trait Personal: Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }
    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

impl Personal for Customer<PersonalPlan> {
    fn advance_feature(&self) {
        println!("Dear {}(id: {}) advanced feature", self.name, self.id);
    }
}

struct FreePlan;
struct PersonalPlan(f32);

impl<T> Customer<T> {
    fn new(name: String) -> Self {
        Customer {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(c: Customer<FreePlan>) -> Self {
        Self::new(c.name)
    }
}

fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan> {
    let _plan = PersonalPlan(payment);
    // 存储 db

    customer.into()
}

#[test]
fn t_customer() {
    // 免费用户
    let customer = Customer::<FreePlan>::new("Try".into());

    // 使用功能
    customer.feature1();
    customer.feature2();

    let customer = subscribe(customer, 6.99);
    customer.feature1();
    customer.feature2();
    // 解锁的新功能
    customer.advance_feature();
}
