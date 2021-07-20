use nom::bytes::complete::take_while;
use nom::Err::Error;
use nom::Finish;
use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_until},
    character::complete::char,
    error::{ErrorKind, ParseError},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};
use regex::{Error as ReError, Regex, RegexBuilder};

#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
    UnknownCommand(char),
    UnknownFlag(char),
    TooManySegments,
    NotEnoughSegments,
    RegexError(ReError),
    Nom(I, ErrorKind),
}

// #[derive(Debug, PartialEq)]
// pub enum CustomError<I> {
//     MyError,
//     Nom(I, ErrorKind),
// }

impl<I> ParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CustomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug, PartialEq)]
pub struct RegexData {
    pattern_str: String,
    pub replace_str: String,
    pub flag_global: bool,
    flag_case_insensitive: bool,
}

// ParseError
// From

// 分隔符可以为多种
// gi 没有使用 combinator
pub fn split_regex(input: &str) -> IResult<&str, RegexData, CustomError<&str>> {
    let (input, _) = tag("s")(input)?;
    let (input, (pattern_str, replace_str)) = pair(
        preceded(tag("/"), take_until("/")),
        preceded(tag("/"), take_while(|x| x != '/')),
    )(input)?;

    let mut ret = RegexData {
        pattern_str: pattern_str.to_owned(),
        replace_str: replace_str.to_string(),
        flag_global: false,
        flag_case_insensitive: false,
    };
    if input == "" {
        return Ok((input, ret));
    }
    let (input, _) = tag("/")(input)?;
    for c in input.chars() {
        match c {
            'i' => ret.flag_case_insensitive = true,
            'g' => ret.flag_global = true,
            _ => return Err(Error(CustomError::UnknownFlag(c))),
        }
    }

    Ok(("", ret))
}

fn parse(input: &str) -> IResult<&str, &str> {
    take_while(|x| x != '/')(input)
}

#[test]
fn test_split() {
    assert_eq!(
        split_regex("s/abc/efg"),
        Ok((
            "",
            RegexData {
                pattern_str: "abc".to_owned(),
                replace_str: "efg".to_owned(),
                flag_global: false,
                flag_case_insensitive: false,
            }
        ))
    );
    assert_eq!(
        split_regex("s/abc/efg/gi"),
        Ok((
            "",
            RegexData {
                pattern_str: "abc".to_owned(),
                replace_str: "efg".to_owned(),
                flag_global: true,
                flag_case_insensitive: true,
            }
        ))
    );
    assert_eq!(
        split_regex("s/abc/efg/a"),
        Err(Error(CustomError::UnknownFlag('a')))
    )
}

#[test]
fn test_m() {
    let input = "foo";
    let (input, s) = parse(input).unwrap();

    // let (input, s) = take_while::<&str>(|x| x != '/')(input).unwrap();
    println!("{}, {}", input, s);
}
