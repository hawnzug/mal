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

#[inline(always)]
pub fn eval(orgast: MalType, mut orgenv: &mut Env) -> MalType {
    let mut ast = orgast;
    let mut env = orgenv.clone();
    loop {
        match ast.clone() {
            MalType::List(ref v) => {
                if v.is_empty() {
                    return MalType::List(vec![]);
                } else {
                    if let MalType::Symbol(ref s) = v[0] {
                        if s == "define" {
                            if v.len() == 3 {
                                if let MalType::Symbol(ref sym) = v[1] {
                                    orgenv.delete(sym);
                                    let second = eval(v[2].clone(), &mut orgenv);
                                    orgenv.set(sym.to_string(), second);
                                    return MalType::Nil;
                                } else {
                                    return MalType::Error("define: first parameter should be a \
                                                           symbol"
                                                              .to_string());
                                }
                            } else {
                                return MalType::Error("define need two parameters".to_string());
                            }
                        } else if s == "lambda" {
                            if v.len() == 3 {
                                return MalType::MalFunc(Box::new(v[1].clone()),
                                                        Box::new(v[2].clone()),
                                                        env);
                            } else {
                                return MalType::Error("lambda need two parameters".to_string());
                            }
                        } else if s == "if" {
                            if v.len() == 4 {
                                match eval(v[1].clone(), &mut env) {
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
                        } else if s == "quote" {
                            return v[1].clone();
                        } else {
                            let mut para = Vec::new();
                            for i in v {
                                para.push(eval(i.clone(), &mut env));
                            }
                            let head = para.remove(0);
                            match head {
                                MalType::Func(f) => return f(para),
                                MalType::MalFunc(formals, body, mut old) => {
                                    // let mut old = oldenv.clone();
                                    old.extend(&mut env);
                                    // let newenv = old.multibind(*formals, para);
                                    match old.multibind(*formals, para) {
                                        // match newenv {
                                        Ok(newnew) => {
                                            ast = *body.clone();
                                            env = newnew.clone();
                                            continue;
                                        }
                                        Err(err) => return err,
                                    };
                                }
                                _ => {
                                    return MalType::Error("The head should be function".to_string())
                                }
                            }
                        }
                    } else {
                        let mut para = Vec::new();
                        for i in v {
                            para.push(eval(i.clone(), &mut env));
                        }
                        let head = para.remove(0);
                        match head {
                            MalType::Func(f) => return f(para),
                            MalType::MalFunc(formals, body, mut old) => {
                                // let mut old = oldenv.clone();
                                old.extend(&mut env);
                                // let newenv = old.multibind(*formals, para);
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
            }
            MalType::Symbol(ref s) => {
                match env.find(&s) {
                    Some(t) => return t,
                    None => return MalType::Error("cannot found".to_string()),
                }
            }
            _ => return ast.clone(),
        }
    }
}

// fn apply(func: &MalType, para: Vec<MalType>, mut env: &mut Env) -> MalType {
//     match *func {
//         // MalType::Func(f) => f(para),
//         MalType::MalFunc(ref formals, ref body, ref oldenv) => {
//             let mut old = oldenv.clone();
//             old.extend(&mut env);
//             let newenv = old.multibind(*formals.clone(), para);
//             match newenv {
//                 Ok(mut newnew) => eval(&**body, &mut newnew),
//                 Err(err) => err,
//             }
//         }
//         _ => MalType::Error("The head should be function".to_string()),
//     }
// }
