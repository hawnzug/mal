#[macro_use]
extern crate chomp;

pub mod steps;
pub mod reader;
pub mod types;
pub mod printer;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    loop {
        print!("λ> ");
        io::stdout().flush().expect("Cannot flush");
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                // let s = steps::step0_repl::rep(&input);
                let s = steps::step1_read_print::rep(&input);
                println!("{}", s);
            }
            Err(error) => println!("{}", error),
        }
        input.clear();
    }
}
