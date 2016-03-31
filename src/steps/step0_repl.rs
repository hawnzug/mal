pub fn read(s: &str) -> &str {
    s
}

pub fn eval(s: &str) -> &str {
    s
}

pub fn print(s: &str) -> &str {
    s
}

pub fn rep(s: &str) -> &str {
    print(eval(read(s)))
}
