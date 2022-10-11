use super::this::SymbolRule;

#[inline]
pub fn new_token(raw: String, position: TokenPosition) -> Token {
    Token {
        ty: match raw.chars().nth(0).unwrap() {
            'a'..='z' | 'A'..='Z' => { match raw.as_str() {
                "true" | "false" => TokenType::Boolean(raw.parse().unwrap()),
                "of" | "end" | "int" | "char" | "bool" | "string" | "float" =>
                    TokenType::Keyword(raw.clone()),
                _ => TokenType::Identifier(raw.clone())
            } },
            '0'..='9' => { if raw.find('.') != None {
                TokenType::Float(raw.parse().unwrap())
            } else {
                TokenType::Number(raw.clone())
            } },
            '"' => TokenType::String(raw.parse().unwrap()),
            '\'' => TokenType::Char(raw.chars().nth(0).unwrap()),
            '`' => TokenType::Atom(raw.clone()[1..raw.len()-1].to_string()),
            ' ' => TokenType::Space,
            '\r' => TokenType::Newline,
            '/' => if raw.starts_with("//") {
                TokenType::Comment
            } else {
                TokenType::Symbol(raw.clone())
            },
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
    Number(String),
    Float(f64),
    Char(char),
    String(String),
    Boolean(bool),
    Identifier(String),
    Atom(String),
    Keyword(String),
    Symbol(String),

    Space,
    Newline,
    End,
    Comment
}

#[derive(Debug, Clone)]
pub struct TokenIterator {
    pub token: Option<Token>,
    pub index: usize,
    pub raw: String,
    pub rules: Vec<SymbolRule>
}