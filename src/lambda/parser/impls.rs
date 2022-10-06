use crate::lambda::reader::this::ReaderTree;

use super::this::Parser;

impl Parser {
    pub fn new(tree: ReaderTree) -> Parser {
        Parser { tree }
    }
}