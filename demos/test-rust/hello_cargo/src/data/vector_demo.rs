use std::collections::HashMap;

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
