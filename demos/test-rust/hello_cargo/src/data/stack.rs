#![allow(dead_code)]

// https://github.com/QMHTMY/RustBook/tree/main/code/chapter03
// https://www.geeksforgeeks.org/data-structure-gq/stack-gq/

struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            top: 0,
            data: Vec::new(),
        }
    }
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.top += 1;
    }
    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }
    fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }
    fn is_empty(&self) -> bool {
        0 == self.top
    }
    fn size(&self) -> usize {
        self.top
    }
}

// 匹配括号
fn pair(source: &str) -> bool {
    let char_list: Vec<char> = source.chars().collect();
    let mut index = 0;
    let mut stack = Vec::new();
    let mut balance = true;
    while index < char_list.len() && balance {
        let c = char_list[index];
        if c == '(' {
            stack.push(c);
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                stack.pop();
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}

fn pair3(source: &str) -> bool {
    let char_list: Vec<char> = source.chars().collect();
    let mut index = 0;
    let mut stack = Vec::new();
    let mut balance = true;
    let opens = "([{";
    let closes = ")]}";
    while index < char_list.len() && balance {
        let c = char_list[index];
        if opens.contains(c) {
            stack.push(c);
        }
        if closes.contains(c) {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                // 索引不一致。肯定不是成对的括号
                if opens.find(top) != closes.find(c) {
                    balance = false
                }
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}

fn t_pair() {
    let sa = "()(())";
    let sb = "()((()";
    assert_eq!(pair(sa), true);
    assert_eq!(pair(sb), false);
    assert_eq!(Some(1), Some(1));
    assert_eq!(Some(1) == Some(1), true);
}

fn t_pair3() {
    let sa = "(2+3){func}[abc]";
    let sb = "(2+3)*(3-1";
    assert_eq!(pair3(sa), true);
    assert_eq!(pair3(sb), false);
}

fn base_converter(dec_num: u32, base: u32) -> String {
    let mut res = String::new();
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut dec_num = dec_num;
    let mut rem_stack = Vec::new();
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        res += &digits[rem].to_string();
    }

    res
}

fn prec(c: char) -> usize {
    match c {
        '^' => 3,
        '/' | '*' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

// https://www.geeksforgeeks.org/stack-set-2-infix-to-postfix/
fn infix_to_postfix(infix: &str) -> Option<String> {
    if !pair3(infix) {
        return None;
    }
    let mut st = Vec::new();
    let mut result = String::new();
    for c in infix.chars() {
        // 使用 match 需要这么写 'a'..='z' | 'A'..='Z'
        if c.is_alphanumeric() {
            result.push(c);
        } else if c == '(' {
            st.push(c);
        } else if c == ')' {
            while st[st.len() - 1] != '(' {
                result.push(st[st.len() - 1]);
                st.pop();
            }
            st.pop();
        } else {
            while !st.is_empty() && prec(c) <= prec(*st.last().unwrap()) {
                if c == '^' && *st.last().unwrap() == '^' {
                    break;
                } else {
                    result.push(st.pop().unwrap())
                }
            }
            st.push(c);
        }
    }
    while !st.is_empty() {
        result.push(st.pop().unwrap())
    }

    Some(result)
}

fn main() {
    t_pair();
    t_pair3();
    // parseInt('2b', 16) ---> 43
    // (43).toString(16) ---> 2b
    assert_eq!(base_converter(10, 2), "1010");
    assert_eq!(base_converter(43, 16), "2B");
    let s = "a+b*(c^d-e)^(f+g*h)-i";
    assert_eq!(infix_to_postfix(s).unwrap(), "abcd^e-fgh*+^*+i-");
}
