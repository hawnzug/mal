use chomp::*;
use std::str;
use types::MalType;

fn to_malint(c: &[u8]) -> MalType {
    let i = str::from_utf8(c).unwrap();
    let j: i32 = i.to_string().parse().unwrap();
    MalType::Int(j)
}

fn parse_int(i: Input<u8>) -> U8Result<MalType> {
    parse!{i;
        let t = take_while1(|c| c >= b'0' && c <= b'9');
        ret to_malint(t)
    }
}

pub fn read_form(s: &str) -> MalType {
    parse_only(parse_int, s.as_bytes()).unwrap()
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
fn test_to_malint() {
    let v = [b'1', b'2'];
    let i = match to_malint(&v) {
        MalType::Int(x) => x,
        _ => 0,
    };
    assert_eq!(i, 12);
}

#[test]
fn test_read_form() {
    let j = match read_form("123") {
        MalType::Int(x) => x,
        _ => 0,
    };
    assert_eq!(123, j);
}
