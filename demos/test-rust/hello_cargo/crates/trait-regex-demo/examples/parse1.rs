use regex::Regex;
use std::str::FromStr;

trait Parse {
    // 关联类型
    type Error;
    // 限定编译时的大小 Sized
    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

// 使用泛型参数 Default
// FromStr
impl<T> Parse for T
where
    T: FromStr + Default,
{
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^\d+(\.\d+)?").unwrap();
        // 根据上下文 Self 推导

        if let Some(caps) = re.captures(s) {
            caps.get(0)
                .map_or(Err("failed to capture".to_string()), |s| {
                    s.as_str()
                        .parse()
                        .map_err(|_| "failed to parse".to_string())
                })
        } else {
            Err("Failed to parse string".to_string())
        }
    }
}
fn main() {
    let _my_u8: u8 = "42".parse().unwrap();

    println!("{}", "42".parse::<i32>().unwrap());
    println!("result: {:?}", u8::parse("255 afdfaf"));
}
