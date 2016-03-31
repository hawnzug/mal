use types::MalType;

pub fn pr_str(m: MalType) -> String {
    match m {
        MalType::Int(x) => x.to_string(),
        _ => "Todo".to_string(),
    }
}

#[test]
fn test_pr_str() {
    assert_eq!("123", pr_str(MalType::Int(123)));
}
