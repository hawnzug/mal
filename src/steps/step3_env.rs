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
                if let MalType::Symbol(ref s) = v[0].clone() {
                    if s == "define" {
                        if v.len() == 3 {
                            if let MalType::Symbol(ref sym) = v[1].clone() {
                                let second = eval(v[2].clone(), env);
                                env.set(sym.to_string(), second);
                                MalType::Nil
                            } else {
                                MalType::Error("define: first parameter should be a symbol"
                                                   .to_string())
                            }
                        } else {
                            MalType::Error("define need two parameters".to_string())
                        }
                    } else if s == "lambda" {
                        if v.len() == 3 {
                            MalType::MalFunc(Box::new(v[1].clone()),
                                             Box::new(v[2].clone()),
                                             env.clone())
                        } else {
                            MalType::Error("lambda need two parameters".to_string())
                        }
                    } else if s == "if" {
                        if v.len() == 4 {
                            match eval(v[1].clone(), env) {
                                MalType::False | MalType::Nil => eval(v[3].clone(), env),
                                err@MalType::Error(_) => err,
                                _ => eval(v[2].clone(), env),
                            }
                        } else {
                            MalType::Error("if: needs 3 parameters".to_string())
                        }
                    } else if s == "quote" {
                        v[1].clone()
                    } else {
                        let mut para = Vec::new();
                        for i in v {
                            para.push(eval(i, env));
                        }
                        let head = para.remove(0);
                        apply(head, para)
                    }
                } else {
                    let mut para = Vec::new();
                    for i in v {
                        para.push(eval(i, env));
                    }
                    let head = para.remove(0);
                    apply(head, para)
                }
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
        MalType::MalFunc(formals, body, env) => {
            let newenv = env.multibind(*formals, para);
            match newenv {
                Ok(mut newnew) => eval(*body, &mut newnew),
                Err(err) => err,
            }
        }
        _ => MalType::Error("The head should be function".to_string()),
    }
}
