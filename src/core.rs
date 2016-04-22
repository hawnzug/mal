use env::Env;
use types::MalType;

fn add(v: Vec<MalType>) -> MalType {
    if v.len() < 2 {
        return MalType::Error("add need 2 parameter".to_string());
    }
    let mut sum: i32 = 0;
    let mut err = false;
    for i in &v {
        match i {
            &MalType::Int(x) => sum += x,
            _ => {
                err = true;
                break;
            }
        };
    }
    if err {
        MalType::Error("add should receive int".to_string())
    } else {
        MalType::Int(sum)
    }
}

fn add1(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("add1 need 1 parameter".to_string())
    } else {
        if let MalType::Int(x) = v[0] {
            MalType::Int(x + 1)
        } else {
            MalType::Error("add1 should receive int".to_string())
        }
    }
}

fn is_null(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("null? need 1 parameter".to_string())
    } else {
        if let MalType::List(ref lst) = v[0] {
            if lst.is_empty() {
                MalType::True
            } else {
                MalType::False
            }
        } else {
            MalType::Error("null? should receive list".to_string())
        }
    }
}

fn car(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("car only needs 1 parameter".to_string())
    } else {
        if let MalType::List(ref lst) = v[0] {
            if lst.is_empty() {
                MalType::Error("car should not take an empty list".to_string())
            } else {
                lst[0].clone()
            }
        } else {
            MalType::Error("car should receive list".to_string())
        }
    }
}

fn cdr(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("cdr only needs 1 parameter".to_string())
    } else {
        if let MalType::List(ref lst) = v[0] {
            if lst.is_empty() {
                MalType::Error("cdr should not take an empty list".to_string())
            } else {
                let mut cdrlist = lst.clone();
                cdrlist.remove(0);
                MalType::List(cdrlist)
            }
        } else {
            MalType::Error("cdr should receive list".to_string())
        }
    }
}

pub fn init_env() -> Env {
    let mut repl_env = Env::new();
    repl_env.set("+".to_string(), MalType::Func(add));
    repl_env.set("add1".to_string(), MalType::Func(add1));
    repl_env.set("null?".to_string(), MalType::Func(is_null));
    repl_env.set("car".to_string(), MalType::Func(car));
    repl_env.set("cdr".to_string(), MalType::Func(cdr));
    repl_env
}
