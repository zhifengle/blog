pub fn knuth_morris_pratt(st: String, pat: String) -> i32 {
    if pat.is_empty() {
        return 0;
    }
    let m = st.len();
    let n = pat.len();
    if m < n {
        return -1;
    }
    // as_bytes  &[u8]
    let st = st.into_bytes();
    let pat = pat.into_bytes();
    let mut next = vec![0; n];
    let mut j = 0;
    // 部分匹配表
    for i in 1..n {
        while j > 0 && pat[i] != pat[j] {
            j = next[j - 1];
        }
        if pat[i] == pat[j] {
            j += 1;
        }
        next[i] = j;
    }
    // reset j;
    j = 0;
    for i in 0..m {
        while j > 0 && st[i] != pat[j] {
            j = next[j - 1]
        }
        if st[i] == pat[j] {
            j += 1;
        }
        if j == n {
            return (i - n + 1) as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str2() {
        assert_eq!(knuth_morris_pratt("hello".to_string(), "ll".to_string()), 2);
    }
}
