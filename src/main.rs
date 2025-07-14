mod lexer;
mod parser;
mod interpritator;

use std::fs::read_to_string;

use crate::interpritator::interpritator::Interpritator;
use crate::lexer::lexer::Lexer;
use crate::lexer::token;
use crate::parser::nodes::*;
use crate::parser::parser::Parser;
use crate::interpritator::objects::Object;

fn main() {
    let mut lex = Lexer::new();
    let mut par = Parser::new();
    let mut int = Interpritator::new();

    for line in read_lines("examples/hello_world.txt") {
        let tokens = lex.token_nize(line);
        if !tokens.is_empty(){
            let node = par.parse(tokens);
            int.execute(node);
        }
    }

}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}