fn i14_temporary_mut() {
    // 在 block 里面临时修改后，在后面保存不变性
    let data = {
        let mut data = get_vec();
        data.sort();
        data
    };
}
fn get_vec() -> Vec<u8> {
    vec![4, 1, 2]
}
