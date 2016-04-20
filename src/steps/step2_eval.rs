use types::MalType;
use reader::read_str;
use printer::pr_str;
use std::collections::HashMap;

pub fn read(s: &str) -> MalType {
    read_str(s)
}

pub fn print(m: MalType) -> String {
    pr_str(&m)
}

pub fn rep(s: &str, env: &mut HashMap<String, MalType>) -> String {
    print(eval(read(s), env))
}

pub fn eval(ast: MalType, env: &mut HashMap<String, MalType>) -> MalType {
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
            match env.get(&s) {
                Some(t) => (*t).clone(),
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

// fn eval_ast(ast: MalType, env: &mut HashMap<String, MalType>) -> MalType {
//     match ast {
//         MalType::Symbol(s) => {
//             match env.get(&s) {
//                 Some(t) => (*t).clone(),
//                 None => MalType::Error("cannot found".to_string()),
//             }
//         }
//         // MalType::List(v) => {
//         //     let mut result = Vec::new();
//         //     for i in v {
//         //         result.push(eval(i, env));
//         //     }
//         //     MalType::List(result)
//         // }
//         _ => ast,
//         // _ => MalType::Error("cannot found".to_string()),
//     }
// }
