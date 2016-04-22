#[macro_use]
extern crate chomp;

pub mod steps;
pub mod reader;
pub mod types;
pub mod printer;
pub mod env;
pub mod core;

use std::io;
use std::io::prelude::*;
use core::init_env;

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
