#![allow(dead_code)]

use std::{
    cell::RefCell,
    fmt,
    rc::Rc,
    slice,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Clone, Copy)]
struct RawBuffer {
    // 裸指针 *const / *mut
    ptr: *mut u8,
    len: usize,
}

// Copy 本身是一个标记 trait
// impl Copy for Foo {}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

// Copy 和 Drop 冲突
// 不实现 Drop 有内存泄漏
// 使用 Drop 会导致 use after free

// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data = unsafe { Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len)) };
//         drop(data)
//     }
// }

impl fmt::Debug for RawBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);
    // buf 是move 的，不用写 drop，也会被 drop
    // drop(buf)
}

fn rc_is_not_send_and_sync() {
    // TV， 最后离开的人关闭
    let a = Rc::new(1);
    let b = a.clone();
    let c = a.clone();
    // `Rc<i32>` cannot be sent between threads safely
    // thread::spawn(move || {
    //     println!("c= {:?}", c);
    // });
}

fn refcell_is_not_sync() {
    // RefCell 内部可变性
    let a = Arc::new(RefCell::new(1));
    let b = a.clone();
    let c = a.clone();
    // thread::spawn(move || {
    //     println!("c= {:?}", c);
    // });
}

fn arc_mutex_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();
    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 1
    });
    {
        let mut g = b.lock().unwrap();
        *g += 1
    }
    handle.join().unwrap();
    println!("a = {:?}", a);
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let buf: RawBuffer = data.into();
    // drop buf 内存
    use_buffer(buf);

    println!("buf: {:?}", buf);
    arc_mutex_is_send_sync();
}
