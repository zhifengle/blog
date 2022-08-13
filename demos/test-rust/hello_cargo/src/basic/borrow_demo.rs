#[test]
fn t_borrow() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1 {:p}",
        &data, data1, &&data, &data1
    );
    println!("--------------------------------");
    println!("sum of data: {}", sum(data1));
    println!("================================");
    println!("sum of data: {}", sum2(data1));
    println!("================================");
    println!(
        "[ {:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    println!(
        "addr of value: {:p}, addr of ref: {:p}, ptr value: {:p}",
        data,
        &data,
        data.as_ptr()
    );
    println!(
        "[ {:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
    data.iter().fold(0, |acc, x| acc + x)
}

fn sum2(data: &[u32]) -> u32 {
    // 切片的值的地址和 Vec 的借用不一样??
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    println!(
        "[ {:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
    data.iter().fold(0, |acc, x| acc + x)
}
