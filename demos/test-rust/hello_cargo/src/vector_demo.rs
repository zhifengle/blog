#[allow(dead_code)]
fn vector_demo() {
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
}
