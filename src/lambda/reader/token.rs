use super::this::SymbolRule;

#[inline]
pub fn new_token(raw: String, position: TokenPosition) -> Token {
    Token {
        ty: match raw.chars().nth(0).unwrap() {
            'a'..='z' | 'A'..='Z' => { match raw.as_str() {
                "true" | "false" => TokenType::Boolean(raw.parse().unwrap()),
                "of" | "end" | "print" | "int" | "char" | "bool" | "string" | "float" =>
                    TokenType::Keyword(raw.clone()),
                _ => TokenType::Identifier(raw.clone())
            } },
            '0'..='9' => { if raw.find('.') != None {
                TokenType::Float(raw.parse().unwrap())
            } else {
                TokenType::Number(raw.parse().unwrap())
            } },
            '"' => TokenType::String(raw.parse().unwrap()),
            '\'' => TokenType::Char(raw.chars().nth(0).unwrap()),
            ' ' | '\n' => TokenType::Space,
            _ => TokenType::Symbol(raw.clone())
        },
        raw,
        position
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub raw: String,
    pub position: TokenPosition
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenPosition {
    pub start: u64,
    pub end: u64,
    pub line: u64
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(u128),
    Float(f64),
    Char(char),
    String(String),
    Boolean(bool),
    Identifier(String),
    Keyword(String),
    Symbol(String),

    Space
}

#[derive(Debug, Clone)]
pub struct TokenIterator {
    pub token: Option<Token>,
    pub index: usize,
    pub raw: String,
    pub rules: Vec<SymbolRule>
}