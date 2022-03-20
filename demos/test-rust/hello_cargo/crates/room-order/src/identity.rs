// 共性的身份: 登录 退出
// 特性

// C++ 身份基类

pub trait Identity {
    fn operate_menu();
}

// @TODO 使用 PhantomData??

pub struct Student {
    name: String,
    pwd: String,
    id: u32,
}

impl Student {
    fn apply_order(&self) {
        unimplemented!()
    }
    fn show_my_order(&self) {
        unimplemented!()
    }
    fn show_all_order(&self) {
        unimplemented!()
    }
    fn cancel_order(&self) {
        unimplemented!()
    }
}
