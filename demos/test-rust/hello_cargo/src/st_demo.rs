#![allow(unused_imports)]
#![allow(dead_code)]

// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// 不能使用 &str 替代 String; 会有生命周期和所有权的问题
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        let _user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
    }
}
