use std::env;

use ansi_term::Colour;

mod lexer;
mod token;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 && args[1] == "pearl" {
        let file_path = &args[2];

        let expressions = utils::utils::get_file_contents(file_path);

        let tokens = lexer::tokenize(&expressions);

        utils::utils::print_tokens(&tokens);
    } else {
        println!("{}", Colour::Red.paint("pearl <path>"))
    }
}
