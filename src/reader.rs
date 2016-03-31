use chomp::*;
use chomp::ascii::is_alpha;
use chomp::ascii::is_digit;
use chomp::ascii::skip_whitespace;
use std::str::from_utf8;
use types::MalType;

pub fn read_str(s: &str) -> MalType {
    parse_only(parse_int, s.as_bytes()).unwrap()
}

fn parse_int(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        let t = take_while1(|c| c >= b'0' && c <= b'9');
        ret to_malint(t)
    }
}

fn parse_atom(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        let first = satisfy(|c| is_symbol(c) || ascii::is_alpha(c));
        let later = take_while1(|c| is_symbol(c)
                                || is_alpha(c)
                                || is_digit(c));
        ret MalType::Symbol(from_utf8(&[first]).unwrap().to_string()+
                            from_utf8(later).unwrap())
    }
}

fn parse_list(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        token(b'(');
        let t: Vec<MalType> = many(parse_all);
        token(b')');
        ret MalType::List(t)
    }
}

fn parse_all(i: Input<u8>) -> U8Result<MalType> {
    let r = parser!{
        parse_atom() <|>
        parse_int() <|>
        parse_list()
    };
    parse!{i;
        skip_whitespace();
        let t = r();
        skip_whitespace();
        ret t
    }
}

fn to_malint(c: &[u8]) -> MalType {
    let i = from_utf8(c).unwrap();
    let j: i32 = i.to_string().parse().unwrap();
    MalType::Int(j)
}

fn is_symbol(c: u8) -> bool {
    match c {
        33 => true,
        35...38 => true,
        42 => true,
        43 => true,
        45 => true,
        47 => true,
        58 => true,
        60...64 => true,
        94...95 => true,
        124 => true,
        126 => true,
        _ => false,
    }
}

#[test]
fn test_parse_list() {
    let i = parse_only(parse_all, "   (   123   )  ".as_bytes());
    let j = match i {
        Ok(MalType::List(v)) => v,
        _ => Vec::new(),
    };
    let x = match j.as_slice()[0] {
        MalType::Int(123) => true,
        _ => false,
    };
    assert!(x)
}

#[test]
fn test_parse_int() {
    let i = parse_only(parse_int, "123".as_bytes());
    let j = match i {
        Ok(MalType::Int(x)) => x,
        _ => 0,
    };
    assert_eq!(j, 123);
}

#[test]
fn test_parse_atom() {
    let i = parse_only(parse_atom, "abc_def".as_bytes());
    let j = match i {
        Ok(MalType::Symbol(s)) => s,
        _ => "".to_string(),
    };
    assert_eq!(j, "abc_def");
    let i = parse_only(parse_atom, "1sdfa".as_bytes());
    let j = match i {
        Ok(MalType::Symbol(s)) => s,
        _ => "".to_string(),
    };
    assert_eq!(j, "");
}

#[test]
fn test_to_malint() {
    let v = [b'1', b'2'];
    let i = match to_malint(&v) {
        MalType::Int(x) => x,
        _ => 0,
    };
    assert_eq!(i, 12);
}

#[test]
fn test_read_str() {
    let j = match read_str("123") {
        MalType::Int(x) => x,
        _ => 0,
    };
    assert_eq!(123, j);
}

#[test]
fn test_is_symbol() {
    let symbols = "!$%&|*+-/:<=?>@^_~#".as_bytes();
    for s in symbols {
        assert!(is_symbol(s.clone()));
    }
    assert!(!is_symbol(b'a'));
}
