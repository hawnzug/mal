use env::Env;
use types::MalType;

fn add(v: Vec<MalType>) -> MalType {
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
        MalType::Error("Add should receive int".to_string())
    } else {
        MalType::Int(sum)
    }
}

pub fn init_env() -> Env {
    let mut repl_env = Env::new();
    repl_env.set("+".to_string(), MalType::Func(add));
    repl_env
}
