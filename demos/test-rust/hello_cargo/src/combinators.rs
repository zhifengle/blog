// https://zhuanlan.zhihu.com/p/342525435

#[test]
fn t_and_then() {
    // and_then with Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = |_| Some("some2"); // similar to: let fn_some = |_| -> Option<&str> { Some("some2") };
    let n: Option<&str> = None;
    let fn_none = |_| None;

    // 都是 Some 得到的值是第二个 Some2
    assert_eq!(s1.and_then(fn_some), s2); // Some1 and_then Some2 = Some2
    assert_eq!(s1.and_then(fn_none), n); // Some and_then None = None
    assert_eq!(n.and_then(fn_some), n); // None and_then Some = None
    assert_eq!(n.and_then(fn_none), n); // None1 and_then None2 = None1

    // and_then with Result
    let o1: Result<&str, &str> = Ok("ok1");
    let o2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2"); // similar to: let fn_ok = |_| -> Result<&str, &str> { Ok("ok2") };

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(o1.and_then(fn_ok), o2); // Ok1 and_then Ok2 = Ok2
    assert_eq!(o1.and_then(fn_err), e2); // Ok and_then Err = Err
    assert_eq!(e1.and_then(fn_ok), e1); // Err and_then Ok = Err
    assert_eq!(e1.and_then(fn_err), e1); // Err1 and_then Err2 = Err1
}
