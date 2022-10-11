use crate::lambda::reader::{token::Token, this::SymbolRule};

pub struct StatementIter {
    pub tokens: Vec<Token>,
    pub rules: Vec<SymbolRule>,
    pub idx: usize
}