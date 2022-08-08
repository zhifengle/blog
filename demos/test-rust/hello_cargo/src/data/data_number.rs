#![allow(unused_imports)]
#![allow(dead_code)]

fn float_to_int() {
    let a: f32 = 42.42;

    // 2**16 - 1 ------>  u16

    // 字节转换
    let frankentype: u32 = unsafe { std::mem::transmute(a) };
    // parseInt('01000010001010011010111000010100', 2) === 1110027796
    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) };

    println!("{}", b);
    assert_eq!(a, b);
}

#[allow(arithmetic_overflow)]
fn impossible_add() {
    let a: u8 = 200;
    let b: u8 = 200;
    let c: u8 = a + b;
    println!("200 + 200 = {}", c);
}

fn inspecting_endianness() {
    let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
    let a: i32 = unsafe { std::mem::transmute(big_endian) };
    let b: i32 = unsafe { std::mem::transmute(little_endian) };
    println!("{} vs {}", a, b);
}

#[test]
fn t_int() {
    // float_to_int();
    // impossible_add();
    inspecting_endianness()
}
