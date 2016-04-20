use types::MalType;

pub fn pr_str(m: &MalType) -> String {
    match *m {
        MalType::List(ref v) => {
            let mut s = '('.to_string();
            if v.len() > 0 {
                s.push_str(&pr_str(&v[0]));
                for item in &v[1..] {
                    s.push(' ');
                    s.push_str(&pr_str(item));
                }
            }
            s.push(')');
            s
        }
        MalType::Int(x) => x.to_string(),
        MalType::Symbol(ref s) => s.to_string(),
        MalType::String(ref s) => "\"".to_string() + s + "\"",
        MalType::Error(ref e) => e.to_string(),
        MalType::True => "#t".to_string(),
        MalType::False => "#f".to_string(),
        MalType::Nil => "nil".to_string(),
        _ => "Todo".to_string(),
    }
}

#[test]
fn test_pr_str() {
    assert_eq!("123", pr_str(&MalType::Int(123)));
    assert_eq!("(1 2 3)",
               pr_str(&MalType::List(vec![MalType::Int(1), MalType::Int(2), MalType::Int(3)])));
}
