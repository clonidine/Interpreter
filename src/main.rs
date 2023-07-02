use ansi_term::Colour;

fn main() {
    
    let welcome_message = Colour::Red.paint("Hello, Pearl Interpreter!");

    println!("{}", welcome_message);
}
