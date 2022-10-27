use std::collections::HashMap;

// https://doc.rust-lang.org/std/vec/struct.Vec.html

// https://doc.rust-lang.org/std/primitive.slice.html
// https://doc.rust-lang.org/std/slice/index.html

// 洗牌。或者说随机 list
// https://programming-idioms.org/idiom/10/shuffle-a-list

#[test]
fn t_vec_base() {
    let mut vec = Vec::new();
    vec.extend([1, 2, 3].iter().copied());
    // 添加多个元素
    vec.extend(&[2, 3, 4]);
    println!("{:?}", vec);

    // https://programming-idioms.org/idiom/291/remove-sublist
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);
    // 清空。就是 v.clear()
    v.drain(..);
    assert_eq!(v, &[]);
}

#[test]
fn t_slice_base() {
    let mut vec = Vec::new();
    vec.extend([1, 2, 3].iter().copied());
    // 两者是共享数据内存
    let int_slice = &vec[..];

    // 得到裸指针  *const i32
    let _x_ptr = int_slice.as_ptr();
    // 创建 window; iter.next().is_none()
    let _c = int_slice.chunks(2);
    // 大小不够会报错。需要遍历全部使用 chunk
    let _w = int_slice.windows(2);
    // v.swap(2, 4); 交换元素

    assert_eq!(["hello", "world"].concat(), "helloworld");
    assert_eq!([[1, 2], [3, 4]].concat(), [1, 2, 3, 4]);

    assert_eq!(["hello", "world"].join(" "), "hello world");
}

#[allow(dead_code)]
pub fn vector_demo() {
    let v1 = vec![1, 2, 3];
    // v1 会被转让 ownership
    for v in v1 {
        println!("{}", v + 1);
    }
    //println!("{:?}", v1);  // 报错
    let v2 = vec![1, 2, 3];
    // 这种方式  v 是 i32; 如果 for v in xx,  v 是 &i32
    // iter_mut() 用于修改
    for &v in v2.iter() {
        println!("{}", v > 1);
    }

    // 2 dimension Vec
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    println!("matrix: {}", matrix[1][0]);

    // Vec ----> Map
    let inorder = vec![6, 5, 4, 3, 2, 1];
    // inorder.iter().enumerate() 不会转移所有权
    let mp: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &j)| (j, i)).collect();
    println!("mp: {:?}", mp);

    // sort
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}
