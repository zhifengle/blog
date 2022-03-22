// https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html
#![allow(unused_imports)]
#![allow(dead_code)]

// Post ---> Draft ---> PendingReview  ---> Published

// 为什么使用 trait State?
// Post 不知道任何相关的操作
// 不使用 trait State, Post 的方法里面需要使用 match 来判断状态
trait State {
    // 直接拿走所有权
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 返回的 &str 声明周期和 post 一致
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

pub struct Post {
    // 私有的状态
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        //self.state.unwrap().content(self)
        // as_ref() 是干嘛用的 ?  上面的写法报错。 Option 里面的数据变成的 &T
        self.state.as_ref().unwrap().content(self)
    }
    // 利用了 state.request_review()
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // 最终阶段。直接返回自身
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // 为什么会使用 & ?   &String ---> &str 么
        &post.content
    }
}

#[test]
fn t_blog() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");

    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
