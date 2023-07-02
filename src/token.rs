#[derive(PartialEq, Debug)]
pub enum TokenType {
    KEYWORD,
    NUMBER,
    IDENTIFIER,
}

pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token { token_type, value }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_token_type(&self) -> &TokenType {
        &self.token_type
    }
}
