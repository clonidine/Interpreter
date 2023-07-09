use std::env;

use ansi_term::Colour;
use lexer::lexer::tokenize;
use utils::utils::{get_file_contents, print_tokens};

mod lexer;
mod parser;
mod token;
mod utils;

const MAX_ARGUMENTS: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == MAX_ARGUMENTS && args[1] == "pearl" {
        let file_path = &args[2];

        let expressions = get_file_contents(file_path);

        let tokens = tokenize(&expressions);

        print_tokens(&tokens);
    } else {
        print_usage()
    }
}

fn print_usage() {
    println!("{}", Colour::Red.paint("pearl <path>"))
}
