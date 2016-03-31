use types::MalType;
use reader::read_form;
use printer::pr_str;

pub fn read(s: &str) -> MalType {
    read_form(s)
}

pub fn eval(m: MalType) -> MalType {
    m
}

pub fn print(m: MalType) -> String {
    pr_str(m)
}

pub fn rep(s: &str) -> String {
    print(eval(read(s)))
}
