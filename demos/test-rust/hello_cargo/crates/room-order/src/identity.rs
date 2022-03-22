const ADMIN_FILE: &str = "admin.txt";
const STUDENT_FILE: &str = "student.txt";
const TEACHER_FILE: &str = "teacher.txt";
const ROOM_FILE: &str = "room.txt";
const ORDER_FILE: &str = "order.txt";

// 共性的身份: 登录 退出
// 特性

// C++ 身份基类

pub trait Identity {
    fn operate_menu(&self);
    // ref p292; 黑马程序员|C++教程
    fn login(&self);
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

pub struct Teacher {
    name: String,
    pwd: String,
    id: u32,
}

impl Teacher {
    fn show_all_order(&self) {
        unimplemented!()
    }
    fn valid_order(&self) {
        unimplemented!()
    }
}

pub struct Manager {
    name: String,
    pwd: String,
}

impl Manager {
    fn add_user(&self) {
        unimplemented!()
    }
    fn show_users(&self) {
        unimplemented!()
    }
    fn show_rooms(&self) {
        unimplemented!()
    }
    fn clear_order_record(&self) {
        unimplemented!()
    }
}
