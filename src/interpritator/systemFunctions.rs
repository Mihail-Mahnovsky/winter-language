use crate::Object;
use crate::expressionNode;

use std::io;
use std::process::exit;

pub fn println(args: Vec<Object>) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}

pub fn print(args: Vec<Object>) {
    for arg in args {
        print!("{} ", arg);
    }
}

pub fn scan() -> Object {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");
    let trimmed = input.trim_end().to_string();
    Object::String(trimmed)
}

pub fn quit() {
    exit(0);
}
