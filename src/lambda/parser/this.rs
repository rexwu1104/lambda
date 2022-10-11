use crate::lambda::reader::{this::{ReaderTree, SymbolRule}, token::Token};

use super::iter::StatementIter;

pub struct Parser {
    pub tree: ReaderTree
}

pub fn new_parser(tree: ReaderTree) -> Parser {
    Parser { tree }
}

pub fn new_iter(rules: Vec<SymbolRule>, tokens: Vec<Token>) -> StatementIter {
    StatementIter {
        rules,
        tokens,
        idx: 0
    }
}