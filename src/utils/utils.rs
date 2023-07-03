use std::fs;

use regex::Regex;

use crate::token::token::TokenType;

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

pub fn get_token_type_name(token_type: TokenType) -> String {
    let mut token_type_name = String::new();

    match token_type {
        TokenType::KEYWORD => token_type_name.push_str("KEYWORD"),
        TokenType::NUMBER => token_type_name.push_str("NUMBER"),
        TokenType::IDENTIFIER => token_type_name.push_str("IDENTIFIER"),
    };

    token_type_name
}

#[cfg(test)]
mod utils_test {
    use crate::{
        token::token::TokenType,
        utils::utils::{get_file_contents, get_token_type_name, is_digit, is_letter, is_operator},
    };

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

    #[test]
    fn getting_token_type_name_correctly() {
        let expected_value = "NUMBER";

        let token_type = TokenType::NUMBER;

        let token_type_name = get_token_type_name(token_type);

        assert!(token_type_name.eq(expected_value))
    }
}
