use crate::token::token::Token;
use crate::token::token::TokenType;
use crate::token::token::TokenTypeMapping;
use crate::utils::utils::{is_digit, is_keyword, is_letter, is_operator};

pub fn tokenize(expressions: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let lexemes = collect_lexemes(expressions);
    let mut table_position: usize = 0;

    for lexeme in lexemes {
        table_position += 1;

        let token_type = get_token_type(&lexeme);
        let token = Token::new(token_type, table_position, lexeme);

        tokens.push(token)
    }

    return tokens;
}

fn get_token_type(lexeme: &str) -> TokenType {
    let token_type_mapping = vec![
        TokenTypeMapping {
            token_type: TokenType::NUMBER,
            check_function: is_digit,
        },
        TokenTypeMapping {
            token_type: TokenType::KEYWORD,
            check_function: is_keyword,
        },
        TokenTypeMapping {
            token_type: TokenType::IDENTIFIER,
            check_function: is_letter,
        },
        TokenTypeMapping {
            token_type: TokenType::OPERATOR,
            check_function: is_operator,
        },
    ];

    for mapping in token_type_mapping {
        if (mapping.check_function)(lexeme) {
            return mapping.token_type;
        }
    }

    panic!("Unknown lexeme.")
}

fn collect_lexemes(expressions: &str) -> Vec<String> {
    expressions
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}
