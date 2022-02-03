//! # Brainfuck interpreter
//! Written in Rust
//!
//! ## Interpreter specification
//! - Array size: 30.000
//! - Cell size: u8
//! - EOF: return 0, don't change cell
//!
//! (c) Jonas Everaert, 2022

use std::{env, io};

fn main() {
    let input = std::fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).expect("Couldn't read file.");
    let mut stdout = io::stdout();
    let mut _stdout: Vec<u8> = Vec::new();
    brainfuck::run(&input, &mut stdout);
}
