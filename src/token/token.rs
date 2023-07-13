#[derive(PartialEq, Debug)]
pub struct TextSpan {
    start: usize,
    end: usize,
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum TokenType {
    KEYWORD,
    INTEGER_LITERAL,
    FLOATING_LITERAL,
    STRING_LITERAL,
    IDENTIFIER,
    FUNCTION_DECLARATION,
    OPERATOR,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    len: usize,
    span: TextSpan,
    value: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> TextSpan {
        TextSpan { start, end }
    }
}

impl Token {
    pub fn new(token_type: TokenType, span: TextSpan, value: String) -> Token {
        let len = value.len();

        Token {
            token_type,
            len,
            span,
            value,
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_start_position(&self) -> usize {
        self.span.start
    }

    pub fn get_end_position(&self) -> usize {
        self.span.end
    }

    pub fn get_len(&self) -> usize {
        self.len
    }
}

pub struct TokenTypeMapping<'a> {
    pub token_type: TokenType,
    pub check_function: fn(&'a str) -> bool,
}
