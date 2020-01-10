#![feature(proc_macro_hygiene)]
extern crate dynasmrt;
extern crate dynasm;

use std::io::{Read};
use std::env;
use std::fs::File;

mod parser;
mod operation;
mod compiler;
mod token;
mod tokenizer;
mod optimizer;

use compiler::State;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument, got {}", args.len());
        return;
    }
    let path = args.pop().unwrap();

    let mut f = if let Ok(f) = File::open(&path) {
        f
    } else {
        println!("Could not open file {}", path);
        return;
    };

    let mut buf = Vec::new();
    if f.read_to_end(&mut buf).is_err() {
        println!("Failed to read from file");
        return;
    }

    let tokens = tokenizer::tokenize(&buf);
    let operations = parser::generate_program(&tokens);

    let operations = optimizer::optimize(operations);

    let program = match compiler::compile(&operations) {
        Ok(p) => p,
        Err(e) => panic!(e)
    };

    let mut state = State::new();

    if let Err(e) = program.run(&mut state) {
        panic!(e);
    }
}
