#![allow(unused_imports)]
#![allow(dead_code)]

// https://gist.github.com/jimmychu0807/9a89355e642afad0d2aeda52e6ad2424

// 分割字符串
#[test]
fn t_split() {
    let link = "xxxEpisode/bbbbbb";
    let idx = link.find("Episode/").unwrap();
    format!("start_", &link[idx + 8..])
}

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

    // PCWSTR 是不可变的字符串
    // windows-rs PWSTR 创建
    let _text = String::from("my pwstr literal")
        .encode_utf16()
        .collect::<Vec<u16>>();
    // let mypwstr = PWSTR(text.as_mut_ptr());
}

fn convert_cookie_string() {
    /*
    let mut str = String::new();
    self.iter().for_each(|cookie| {
        str.push_str(&format!("{}={}; ", cookie.name, cookie.value));
    });
    str.pop();
    str.pop();
    */
}
