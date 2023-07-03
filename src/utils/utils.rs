use std::fs;

use regex::Regex;

pub fn is_letter(lexeme: &str) -> bool {
    let regex_letter = Regex::new(r"[A-Z]").unwrap();

    regex_letter.is_match(lexeme)
}

pub fn is_digit(lexeme: &str) -> bool {
    let regex_digit = Regex::new(r"^[-]?\d+(\.\d+)?$").unwrap();

    regex_digit.is_match(lexeme)
}

pub fn is_operator(lexeme: &str) -> bool {
    let regex_operator = Regex::new(r"[\+\-\*/%=\^&|<>!~]").unwrap();

    regex_operator.is_match(lexeme)
}

pub fn get_file_contents(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Please, enter a correct file path.");

    contents
}

#[cfg(test)]
mod utils_test {
    use crate::utils::utils::{get_file_contents, is_digit, is_letter, is_operator};

    #[test]
    fn getting_file_contents_correctly() {
        let file_path = "/home/archlinux/test.pearl";

        let contents = get_file_contents(&file_path);

        let exact_file_content = "int a = 2
int b = 30
int c = 50";

        assert!(contents.eq(exact_file_content));
    }

    #[test]
    fn detecting_operators() {
        let operators = ["=", "-", "+", "<", ">", "*", "!", "%", "/"];

        operators
            .iter()
            .for_each(|operator| assert!(is_operator(operator)))
    }

    #[test]
    fn detecting_letter() {
        let letter = "Hello, World!";

        assert!(is_letter(letter))
    }

    #[test]
    fn detecting_digits() {
        let numbers = ["1234", "-1923", "94.34", "-123.45"];

        numbers.iter().for_each(|number| assert!(is_digit(number)))
    }
}
