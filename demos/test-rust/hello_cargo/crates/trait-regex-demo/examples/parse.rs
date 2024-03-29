use regex::Regex;

trait Parse {
    fn parse(s: &str) -> Self;
}

// xx_opt.map_or(0, |x| x)
impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            // 第一个match
            captures
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

fn main() {
    println!("result: {}", u8::parse("255 hello"));
}
