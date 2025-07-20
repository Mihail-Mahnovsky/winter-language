mod interpritator;
mod lexer;
mod parser;

use std::fs::read_to_string;

use crate::interpritator::interpritator::Interpritator;
use crate::interpritator::objects::Object;
use crate::lexer::lexer::Lexer;
use crate::lexer::token;
use crate::parser::nodes::*;
use crate::parser::parser::Parser;

fn main() {
    let mut lex = Lexer::new();
    let mut par = Parser::new();
    let mut int = Interpritator::new();

    let mut code: String = String::new();

    for line in read_lines("examples/test.wn") {
        code = format!("{}{}\n", code, line);
    }

    let tokens = lex.token_nize(code);
    if !tokens.is_empty() {
        let nodes = par.parse(tokens);
        for node in nodes {
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
