use crate::token::token::TextSpan;
use crate::token::token::Token;
use crate::token::token::TokenType;
use crate::token::token::TokenTypeMapping;
use crate::utils::utils::is_floating_point;
use crate::utils::utils::is_string;
use crate::utils::utils::{is_digit, is_keyword, is_letter, is_operator};

pub fn tokenize(expressions: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let lexemes = collect_lexemes(expressions);
    let mut table_position: usize = 0;

    for lexeme in lexemes {
        table_position += 1;

        let token_type = get_token_type(&lexeme);
        let token = Token::new(
            token_type,
            TextSpan::new(table_position, lexeme.len()),
            lexeme,
        );

        tokens.push(token)
    }

    tokens
}

fn get_token_type(lexeme: &str) -> TokenType {
    let token_type_mapping = vec![
        TokenTypeMapping {
            token_type: TokenType::FLOATING_LITERAL,
            check_function: is_floating_point,
        },
        TokenTypeMapping {
            token_type: TokenType::INTEGER_LITERAL,
            check_function: is_digit,
        },
        TokenTypeMapping {
            token_type: TokenType::STRING_LITERAL,
            check_function: is_string,
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

    panic!("Unknown lexeme. Value: {}", lexeme)
}

fn collect_lexemes(expressions: &str) -> Vec<String> {
    let mut lexemes = Vec::new();
    let mut current_lexeme = String::new();
    let mut inside_string = false;

    for c in expressions.chars() {
        if c == '"' {
            inside_string = !inside_string;
        }

        if !inside_string && c.is_whitespace() {
            if !current_lexeme.is_empty() {
                lexemes.push(current_lexeme.clone());
                current_lexeme.clear();
            }
        } else {
            current_lexeme.push(c);
        }
    }

    if !current_lexeme.is_empty() {
        lexemes.push(current_lexeme);
    }

    lexemes
}

#[cfg(test)]
mod lexer_test {
    use crate::token::token::{TextSpan, Token, TokenType};

    use super::tokenize;

    #[test]
    fn getting_tokens_correctly() {
        let src = r#"
        str hello = "Hello, World!"
        "#;

        let tokens = tokenize(&src);

        let expected_tokens = vec![
            Token::new(TokenType::KEYWORD, TextSpan::new(1, 3), "str".to_string()),
            Token::new(
                TokenType::IDENTIFIER,
                TextSpan::new(2, 5),
                "hello".to_string(),
            ),
            Token::new(TokenType::OPERATOR, TextSpan::new(3, 1), "=".to_string()),
            Token::new(
                TokenType::STRING_LITERAL,
                TextSpan::new(4, 15),
                r#""Hello, World!""#.to_string(),
            ),
        ];

        let mut index: usize = 0;

        while index < tokens.len() {
            let token = tokens.get(index);
            let expected_token = expected_tokens.get(index);

            assert_eq!(token, expected_token);

            index += 1
        }
    }
}
