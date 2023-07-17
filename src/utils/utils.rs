use crate::token::token::{Token, TokenType};
use regex::Regex;
use std::fs;

pub fn is_letter(lexeme: &str) -> bool {
    let regex_letter = Regex::new(r#"^[a-zA-Z_][a-zA-Z0-9_]*$|\".*\""#).unwrap();

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

pub fn is_string(lexeme: &str) -> bool {
    let regex_string = Regex::new(r#""[^"]+""#).unwrap();
    regex_string.is_match(lexeme)
}

pub fn is_integer(lexeme: &str) -> bool {
    let regex_integer = Regex::new(r"^[-+]?\d+$").unwrap();

    regex_integer.is_match(lexeme)
}

pub fn is_floating_point(lexeme: &str) -> bool {
    let regex_floating_point = Regex::new(r"[+-]?\d+\.\d+([eE][+-]?\d+)?").unwrap();

    regex_floating_point.is_match(lexeme)
}

pub fn is_keyword(lexeme: &str) -> bool {
    let keyword_regex = Regex::new(r"\b(abstract|assert|boolean|break|byte|case|catch|char|class|const|continue|default|do|double|else|enum|exports|extends|final|finally|float|for|goto|if|implements|import|instanceof|int|interface|long|module|native|new|opens|package|private|protected|provides|public|requires|return|short|static|strictfp|super|switch|synchronized|this|throw|throws|transient|try|exports|void|volatile|while|with|str)\b").unwrap();

    keyword_regex.is_match(lexeme)
}

pub fn is_function(lexeme: &str) -> bool {
    let function_regex = Regex::new(r"").unwrap();

    function_regex.is_match(lexeme)
}

pub fn get_file_contents(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Please, enter a correct file path.");

    contents
}

pub fn get_token_type_name(token_type: &TokenType) -> String {
    let mut token_type_name = String::new();

    match token_type {
        TokenType::KEYWORD => token_type_name.push_str("KEYWORD"),
        TokenType::INTEGER_LITERAL => token_type_name.push_str("INTEGER_LITERAL"),
        TokenType::FLOATING_LITERAL => token_type_name.push_str("FLOATING_LITERAL"),
        TokenType::STRING_LITERAL => token_type_name.push_str("STRING_LITERAL"),
        TokenType::FUNCTION_DECLARATION => token_type_name.push_str("FUNCTION_DECLARATION"),
        TokenType::IDENTIFIER => token_type_name.push_str("IDENTIFIER"),
        TokenType::OPERATOR => token_type_name.push_str("OPERATOR"),
    };

    token_type_name
}

pub fn print_tokens(tokens: &Vec<Token>) {
    tokens.iter().for_each(|token| {
        println!(
            "Token(type='{}', start='{}', length='{}', value='{}')",
            get_token_type_name(token.get_token_type()),
            token.get_start_position(),
            token.get_len(),
            token.get_value()
        )
    });
}

#[cfg(test)]
mod utils_test {
    use crate::utils::utils::{is_floating_point, is_integer, is_operator, is_string};

    #[test]
    fn detecting_operators() {
        let operators = ["=", "-", "+", "<", ">", "*", "!", "%", "/"];

        operators
            .iter()
            .for_each(|operator| assert!(is_operator(operator)))
    }

    #[test]
    fn detecting_string() {
        let string = r#""Hello, World""#;

        assert!(is_string(string))
    }

    #[test]
    fn detecting_integer() {
        let negative_integer = "-123";
        let positive_integer = "123";

        assert!(is_integer(negative_integer));
        assert!(is_integer(positive_integer))
    }

    #[test]
    fn detecting_floating_point() {
        let numbers = ["123.4", "-123.45"];

        numbers
            .iter()
            .for_each(|number| assert!(is_floating_point(number)))
    }

}
