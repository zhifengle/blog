/// 05
#[allow(dead_code)]
pub fn replace_space(s: String) -> String {
    // 直接调用替换
    // let new_s = s.replace(" ", "%20");
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

#[cfg(test)]
mod tests_05 {
    use super::*;

    #[test]
    fn replace_space_test() {
        let s = "We are happy.";
        let res = replace_space(s.to_string());
        assert_eq!(res, String::from("We%20are%20happy."));
    }
}
