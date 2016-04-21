#[macro_use]
extern crate chomp;

pub mod steps;
pub mod reader;
pub mod types;
pub mod printer;
pub mod env;

use std::io;
use std::io::prelude::*;
use env::Env;
use types::MalType;

fn main() {
    let mut input = String::new();
    let mut repl_env = init_env();
    loop {
        print!("λ> ");
        io::stdout().flush().expect("Cannot flush");
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                // let s = steps::step0_repl::rep(&input);
                let s = steps::step3_env::rep(&input, &mut repl_env);
                println!("{}", s);
            }
            Err(error) => println!("{}", error),
        }
        input.clear();
    }
}

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

fn init_env() -> Env {
    let mut repl_env = Env::new();
    repl_env.set("+".to_string(), MalType::Func(add));
    repl_env
}
