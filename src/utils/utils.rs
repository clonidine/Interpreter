use std::fs;

use regex::Regex;

use crate::token::token::{Token, TokenType};

pub fn is_letter(lexeme: &str) -> bool {
    let regex_letter = Regex::new(r"[A-Za-z]").unwrap();

    regex_letter.is_match(lexeme)
}

pub fn is_digit(lexeme: &str) -> bool {
    let regex_digit = Regex::new(r"^-?\d+(\.\d+)?$").unwrap();

    regex_digit.is_match(lexeme)
}

pub fn is_operator(lexeme: &str) -> bool {
    let regex_operator = Regex::new(r"(^|\s|\()([-+*/%=]|[=!<>]=?|\|\||&&)(\s|\)|$)").unwrap();

    regex_operator.is_match(lexeme)
}

pub fn is_keyword(lexeme: &str) -> bool {
    let keyword_regex = Regex::new(r"\b(abstract|assert|boolean|break|byte|case|catch|char|class|const|continue|default|do|double|else|enum|exports|extends|final|finally|float|for|goto|if|implements|import|instanceof|int|interface|long|module|native|new|opens|package|private|protected|provides|public|requires|return|short|static|strictfp|super|switch|synchronized|this|throw|throws|transient|try|exports|void|volatile|while|with)\b").unwrap();

    keyword_regex.is_match(lexeme)
}

pub fn get_file_contents(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Please, enter a correct file path.");

    contents
}

pub fn get_token_type_name(token_type: &TokenType) -> String {
    let mut token_type_name = String::new();

    match token_type {
        TokenType::KEYWORD => token_type_name.push_str("KEYWORD"),
        TokenType::NUMBER => token_type_name.push_str("NUMBER"),
        TokenType::IDENTIFIER => token_type_name.push_str("IDENTIFIER"),
        TokenType::OPERATOR => token_type_name.push_str("OPERATOR"),
    };

    token_type_name
}

pub fn print_tokens(tokens: &Vec<Token>) {
    tokens.iter().for_each(|token| {
        println!(
            "TOKEN_TYPE='{}', TABLE_POSITION='{}', VALUE='{}'",
            get_token_type_name(token.get_token_type()),
            token.get_table_position(),
            token.get_value()
        )
    });
}

#[cfg(test)]
mod utils_test {
    use crate::{
        token::token::TokenType,
        utils::utils::{get_token_type_name, is_digit, is_letter, is_operator},
    };

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

        let token_type_name = get_token_type_name(&token_type);

        assert!(token_type_name.eq(expected_value))
    }
}
