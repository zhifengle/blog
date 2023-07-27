// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

/// 需要设置 pub, 否则无法编译
// struct Node {
//     elem: i32,
//     next: List,
// }

// pub enum List {
//     Empty,
//     More(Box<Node>),
// }

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Node {
    //         elem: elem,
    //         next: self.head,
    //     };
    // }
    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Box::new(Node {
    //         elem: elem,
    //         next: self.head,
    //     });
    //     self.head = Link::More(new_node);
    // }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    // pub fn pop(&mut self) -> Option<i32> {
    //     // match self.head {  // has error
    //     match &self.head {
    //         Link::Empty => {
    //             // TODO
    //         }
    //         Link::More(node) => {
    //             // TODO
    //         }
    //     };
    //     unimplemented!()
    // }

    // pub fn pop(&mut self) -> Option<i32> {
    //     let result;
    //     match &self.head {
    //         Link::Empty => {
    //             result = None;
    //         }
    //         Link::More(node) => {
    //             result = Some(node.elem);
    //             // want to move node, but can't
    //             self.head = node.next;
    //         }
    //     };
    //     result
    // }

    // use mem::replace;
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
