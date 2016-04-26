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

pub fn rep(s: &str, mut env: &mut Env, mut global: &mut Env) -> String {
    print(eval(read(s), &mut env, global))
}

pub fn eval(orgast: MalType, orgenv: &mut Env, mut global: &mut Env) -> MalType {
    let mut ast = orgast;
    let mut env = orgenv.clone();
    loop {
        let mut apply = false;
        match ast.clone() {
            MalType::List(ref v) => {
                if v.is_empty() {
                    return MalType::List(vec![]);
                }
                if let MalType::Symbol(ref s) = v[0] {
                    match s as &str {
                        "define" => {
                            if v.len() == 3 {
                                if let MalType::Symbol(ref sym) = v[1] {
                                    let second = eval(v[2].clone(), &mut env, global);
                                    global.set(sym.to_string(), second);
                                    return MalType::Nil;
                                } else {
                                    return MalType::Error("define: first parameter should be \
                                                               a symbol"
                                                              .to_string());
                                }
                            }
                        }
                        "lambda" => {
                            if v.len() == 3 {
                                return MalType::MalFunc(Box::new(v[1].clone()),
                                                        Box::new(v[2].clone()),
                                                        env);
                            } else {
                                return MalType::Error("lambda need two parameters".to_string());
                            }
                        }
                        "if" => {
                            if v.len() == 4 {
                                match eval(v[1].clone(), &mut env, global) {
                                    MalType::False | MalType::Nil => {
                                        ast = v[3].clone();
                                        continue;
                                    }
                                    err@MalType::Error(_) => return err,
                                    _ => {
                                        ast = v[2].clone();
                                        continue;
                                    }
                                }
                            } else {
                                return MalType::Error("if: needs 3 parameters".to_string());
                            }
                        }
                        "quote" => return v[1].clone(),
                        _ => apply = true,
                    }
                } else {
                    apply = true;
                }
                if apply {
                    let mut para = Vec::new();
                    for i in v {
                        para.push(eval(i.clone(), &mut env, global));
                    }
                    let head = para.remove(0);
                    match head {
                        MalType::Func(f) => return f(para),
                        MalType::MalFunc(formals, body, old) => {
                            match old.multibind(*formals, para) {
                                Ok(newnew) => {
                                    ast = *body.clone();
                                    env = newnew.clone();
                                    continue;
                                }
                                Err(err) => return err,
                            };
                        }
                        _ => return MalType::Error("The head should be function".to_string()),
                    }
                }
            }
            MalType::Symbol(ref s) => {
                match env.find(&s) {
                    Some(t) => return t,
                    None => {
                        match global.find(&s) {
                            Some(t) => return t,
                            None => return MalType::Error("cannot found".to_string()),
                        }
                    }
                }
            }
            _ => return ast.clone(),
        }
    }
}
