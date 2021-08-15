use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};

fn parens(i: &str) -> IResult<&str, i64> {
    delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
}

fn factor(i: &str) -> IResult<&str, i64> {
    // alt 多个 parser. 支撑 alt 嵌套 alt
    alt((
        map_res(delimited(space0, digit1, space0), FromStr::from_str),
        parens,
    ))(i)
}

// 乘除法优先级更高
fn term(i: &str) -> IResult<&str, i64> {
    let (i, init) = factor(i)?;
    // 第一个参数是 parser
    fold_many0(
        pair(alt((char('*'), char('/'))), factor),
        init,
        |acc, (op, val)| {
            if op == '*' {
                acc * val
            } else {
                acc / val
            }
        },
    )(i)
}

// 加减法
fn expr(i: &str) -> IResult<&str, i64> {
    let (i, init) = term(i)?;
    fold_many0(
        pair(alt((char('+'), char('-'))), term),
        init,
        |acc, (op, val)| {
            if op == '+' {
                acc + val
            } else {
                acc - val
            }
        },
    )(i)
}

#[test]
fn factor_test() {
    assert_eq!(factor("3"), Ok(("", 3)));
    assert_eq!(factor(" 12"), Ok(("", 12)));
    assert_eq!(factor("537  "), Ok(("", 537)));
    assert_eq!(factor("  24   "), Ok(("", 24)))
}

#[test]
fn term_test() {
    assert_eq!(term(" 12 *2 /  3"), Ok(("", 8)));
    assert_eq!(term(" 2* 3  *2 *2 /  3"), Ok(("", 8)));
    assert_eq!(term(" 48 /  3/2"), Ok(("", 8)));
}

#[test]
fn expr_test() {
    assert_eq!(expr(" 1 +  2 "), Ok(("", 3)));
    assert_eq!(expr(" 12 + 6 - 4+  3"), Ok(("", 17)));
    assert_eq!(expr(" 1 + 2*3 + 4"), Ok(("", 11)));
}

#[test]
fn parens_test() {
    assert_eq!(expr(" (  2 )"), Ok(("", 2)));
    assert_eq!(expr(" 2* (  3 + 4 ) "), Ok(("", 14)));
    assert_eq!(expr("  2*2 / ( 5 - 1) + 3"), Ok(("", 4)));
}
