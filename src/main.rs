mod base;
mod parser;
mod tokenizer;
mod tree;

use std::io::{self, Write};
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::tree::*;

fn main() {
    print!("Masukkan ekspresi : ");
    io::stdout().flush().expect("Flush gagal");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Gagal membaca ekspresi");

    let mut tokenizer = Tokenizer::new(&line);
    tokenizer.next_token();

    let mut parser = Parser::new(&mut tokenizer);

    let node = parser.parse_expression(false);

    println!("{}", draw_string(&node, String::new()));

    println!("Evaluated to: {}", evaluate(&node));
}
