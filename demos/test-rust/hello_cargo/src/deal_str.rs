#![allow(unused_imports)]
#![allow(dead_code)]

// C++ char* str, length; 的解法是先算出空格数 *2 + 原长；
// 通过倒序的遍历，遍历原长。 遇到空格倒序填充为 '0' '2' '%'
// 遇到正常字符正常填充
// rust 不支持 string 的索引的。
// let ss = s.chars().nth(i);
// match ss {
//     Some(c) => {
//         if c == ' ' {
//             j -= 1;
//         }
//     }
//     None => (),
// }

#[allow(dead_code)]
pub fn replace_space(s: String) -> String {
    // 支持直接替换
    // let ans = s.replace(' ', "%20");
    let mut new_s = String::new();
    for c in s.chars() {
        if c == ' ' {
            new_s.push_str("%20");
        } else {
            new_s.push(c);
        }
    }
    new_s
}

// https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html
#[test]
fn t_string() {
    let hello = String::from("你好");
    // 2个字符
    for c in hello.chars() {
        println!("{}", c);
    }
    // js 的写法
    // let utf8Encode = new TextEncoder(); utf8Encode.encode('你好');

    // 6 个;  [228, 189, 160, 229, 165, 189]
    for b in hello.bytes() {
        println!("{}", b);
    }

    // windows-rs PWSTR 创建
    let _text = String::from("my pwstr literal")
        .encode_utf16()
        .collect::<Vec<u16>>();
    // let mypwstr = PWSTR(text.as_mut_ptr());
}
