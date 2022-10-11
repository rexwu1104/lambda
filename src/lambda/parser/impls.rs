use super::{iter::StatementIter, tree::Statement, generate::generate_statement};

impl Iterator for StatementIter {
    type Item = Statement;

    fn next(&mut self) -> Option<Self::Item> {
        generate_statement(self)
    }
}