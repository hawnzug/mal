use types::MalType;
use reader::read_str;
use printer::pr_str;
use env::Env;

pub fn read(s: &str) -> MalType {
    read_str(s)
}

pub fn print(m: MalType) -> String {
    pr_str(&m)
}

pub fn rep(s: &str, mut env: &mut Env) -> String {
    print(eval(read(s), &mut env))
}

pub fn eval(ast: MalType, env: &mut Env) -> MalType {
    match ast {
        MalType::List(v) => {
            if v.is_empty() {
                MalType::List(vec![])
            } else {
                let mut para = Vec::new();
                for i in v {
                    para.push(eval(i, env));
                }
                let head = para.remove(0);
                apply(head, para)
            }
        }
        MalType::Symbol(s) => {
            match env.find(&s) {
                Some(t) => t,
                None => MalType::Error("cannot found".to_string()),
            }
        }
        _ => ast,
    }
}

fn apply(func: MalType, para: Vec<MalType>) -> MalType {
    match func {
        MalType::Func(f) => f(para),
        _ => MalType::Error("The head should be function".to_string()),
    }
}
