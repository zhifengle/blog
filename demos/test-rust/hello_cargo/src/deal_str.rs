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
