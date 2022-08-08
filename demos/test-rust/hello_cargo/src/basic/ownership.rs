fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    // function 是一个指针
    is_copy::<fn()>();

    // raw pointer
    is_copy::<*const String>();
    is_copy::<*mut String>();

    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    // u8 实现了 copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // // 不定长的数据没有实现 copy
    // is_copy::<str>();
    // is_copy::<[u8]>();
    // is_copy::<Vec<u8>>();
    // is_copy::<String>();

    // // 能修改的引用，也没有实现 copy
    // is_copy::<&mut String>();

    // // Vec<u8> 本身没有实现 copy
    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u8)>();
}

#[test]
fn t_copy_trait() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}

// 借用没有破坏所有权
// A 拥有 aa,  B 可以借用 aa
// Rust 所有参数传递都是传值

fn sum(data: &[u32]) -> u32 {
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    // data.iter().fold(0, |acc, x| acc + x)
    data.iter().sum()
}

#[test]
fn t_borrow() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 一次运行结果, 下一次不一定一样
    // 0x229b4ff350 (0x229b4ff350), addr of data 0x229b4ff410, data1 0x229b4ff368
    // &data 和 data1 两个数据是一样。但是地址不一样
    // ?? 我觉得 &data data1 应该是和传入 sum 一样. 实际上不一样[]
    println!(
        "addr of value: {:p} ({:p}), addr of data {:p}, data1 {:p}",
        &data, data1, &&data, &data1
    );
    // addr of value: 0x16fbbdda830, addr of ref: 0x229b4ff160
    println!("Sum of data1: {}", sum(data1));

    // [0x16fbbdda830, 0x16fbbdda834, 0x16fbbdda838, 0x16fbbdda83c]
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}
