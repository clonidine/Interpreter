#[derive(PartialEq, Debug)]
pub enum TokenType {
    KEYWORD,
    NUMBER,
    IDENTIFIER,
    OPERATOR,
    UNKNOWN,
}
pub struct Token {
    token_type: TokenType,
    table_position: usize,
    value: String,
}

impl Token {
    pub fn new(token_type: TokenType, table_position: usize, value: String) -> Token {
        Token {
            token_type,
            table_position,
            value,
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_table_position(&self) -> usize {
        self.table_position
    }
}

pub struct TokenTypeMapping<'a> {
    pub token_type: TokenType,
    pub check_function: fn(&'a str) -> bool,
}

