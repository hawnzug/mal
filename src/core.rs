use env::Env;
use types::MalType;

#[inline]
fn add(v: Vec<MalType>) -> MalType {
    if v.len() < 2 {
        return MalType::Error("add need 2 parameter".to_string());
    }
    let mut sum: i32 = 0;
    let mut err = false;
    for i in v {
        match i {
            MalType::Int(x) => sum += x,
            err@MalType::Error(_) => {
                return err;
            }
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

#[inline]
fn sub(v: Vec<MalType>) -> MalType {
    if v.len() < 2 {
        return MalType::Error("sub need 2 parameter".to_string());
    }
    let mut sum = 0;
    let mut err = false;
    if let MalType::Int(x) = v[0] {
        sum = 2 * x;
        for i in v {
            match i {
                MalType::Int(x) => sum -= x,
                err@MalType::Error(_) => {
                    return err;
                }
                _ => {
                    err = true;
                    break;
                }
            };
        }
    } else {
        err = true;
    }
    if err {
        MalType::Error("sub should receive int".to_string())
    } else {
        MalType::Int(sum)
    }
}

#[inline]
fn add1(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("add1 need 1 parameter".to_string())
    } else {
        match v[0] {
            MalType::Int(x) => MalType::Int(x + 1),
            ref err@MalType::Error(_) => err.clone(),
            _ => MalType::Error("add1 should receive int".to_string()),
        }
    }
}

#[inline]
fn sub1(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("sub1 need 1 parameter".to_string())
    } else {
        match v[0] {
            MalType::Int(x) => MalType::Int(x - 1),
            ref err@MalType::Error(_) => err.clone(),
            _ => MalType::Error("sub1 should receive int".to_string()),
        }
    }
}

#[inline]
fn is_zero(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("zero? need 1 parameter".to_string())
    } else {
        match v[0] {
            MalType::Int(0) => MalType::True,
            MalType::Int(_) => MalType::False,
            ref err@MalType::Error(_) => err.clone(),
            _ => MalType::Error("zero? should receive int".to_string()),
        }
    }
}

#[inline]
fn is_null(v: Vec<MalType>) -> MalType {
    if v.len() != 1 {
        MalType::Error("null? need 1 parameter".to_string())
    } else {
        match v[0] {
            MalType::List(ref lst) => {
                if lst.is_empty() {
                    MalType::True
                } else {
                    MalType::False
                }
            }
            ref err@MalType::Error(_) => err.clone(),
            _ => MalType::Error("null? should receive list".to_string()),
        }
    }
}

#[inline]
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

#[inline]
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

#[inline]
fn cons(v: Vec<MalType>) -> MalType {
    if v.len() != 2 {
        MalType::Error("cons needs 2 parameter".to_string())
    } else {
        if let MalType::Error(_) = v[0] {
            v[0].clone()
        } else {
            if let MalType::List(ref lst) = v[1] {
                let mut conslist = Vec::new();
                conslist.push(v[0].clone());
                for i in lst {
                    conslist.push(i.clone());
                }
                MalType::List(conslist)
            } else {
                MalType::Error("cons should receive list".to_string())
            }
        }
    }
}

#[inline]
fn equal(v: Vec<MalType>) -> MalType {
    if v.len() != 2 {
        MalType::Error("= only needs 2 parameter".to_string())
    } else {
        if let MalType::Int(a) = v[0] {
            if let MalType::Int(b) = v[1] {
                if a == b {
                    MalType::True
                } else {
                    MalType::False
                }
            } else {
                MalType::Error("= should receive int".to_string())
            }
        } else {
            MalType::Error("= should receive int".to_string())
        }
    }
}

#[inline]
fn less(v: Vec<MalType>) -> MalType {
    if v.len() != 2 {
        MalType::Error("< only needs 2 parameter".to_string())
    } else {
        if let MalType::Int(a) = v[0] {
            if let MalType::Int(b) = v[1] {
                if a < b {
                    MalType::True
                } else {
                    MalType::False
                }
            } else {
                MalType::Error("< should receive int".to_string())
            }
        } else {
            MalType::Error("< should receive int".to_string())
        }
    }
}

#[inline]
fn greater(v: Vec<MalType>) -> MalType {
    if v.len() != 2 {
        MalType::Error("> only needs 2 parameter".to_string())
    } else {
        if let MalType::Int(a) = v[0] {
            if let MalType::Int(b) = v[1] {
                if a > b {
                    MalType::True
                } else {
                    MalType::False
                }
            } else {
                MalType::Error("> should receive int".to_string())
            }
        } else {
            MalType::Error("> should receive int".to_string())
        }
    }
}

#[inline]
fn less_equal(v: Vec<MalType>) -> MalType {
    if v.len() != 2 {
        MalType::Error("<= only needs 2 parameter".to_string())
    } else {
        if let MalType::Int(a) = v[0] {
            if let MalType::Int(b) = v[1] {
                if a <= b {
                    MalType::True
                } else {
                    MalType::False
                }
            } else {
                MalType::Error("<= should receive int".to_string())
            }
        } else {
            MalType::Error("<= should receive int".to_string())
        }
    }
}

#[inline]
fn greater_equal(v: Vec<MalType>) -> MalType {
    if v.len() != 2 {
        MalType::Error(">= only needs 2 parameter".to_string())
    } else {
        if let MalType::Int(a) = v[0] {
            if let MalType::Int(b) = v[1] {
                if a >= b {
                    MalType::True
                } else {
                    MalType::False
                }
            } else {
                MalType::Error(">= should receive int".to_string())
            }
        } else {
            MalType::Error(">= should receive int".to_string())
        }
    }
}

pub fn init_env() -> Env {
    let mut repl_env = Env::new();
    repl_env.set("+".to_string(), MalType::Func(add));
    repl_env.set("add1".to_string(), MalType::Func(add1));
    repl_env.set("-".to_string(), MalType::Func(sub));
    repl_env.set("sub1".to_string(), MalType::Func(sub1));
    repl_env.set("null?".to_string(), MalType::Func(is_null));
    repl_env.set("zero?".to_string(), MalType::Func(is_zero));
    repl_env.set("car".to_string(), MalType::Func(car));
    repl_env.set("cdr".to_string(), MalType::Func(cdr));
    repl_env.set("cons".to_string(), MalType::Func(cons));
    repl_env.set("=".to_string(), MalType::Func(equal));
    repl_env.set("<".to_string(), MalType::Func(less));
    repl_env.set(">".to_string(), MalType::Func(greater));
    repl_env.set("<=".to_string(), MalType::Func(less_equal));
    repl_env.set(">=".to_string(), MalType::Func(greater_equal));
    repl_env
}
