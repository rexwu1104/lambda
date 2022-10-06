use crate::lambda::reader::{methods::{get_vec, define_operator}, this::{Reader, ReaderTree, SymbolRule}, token::{Token, TokenType, new_token, TokenPosition}};

use super::this::Parser;

use std::thread;

pub fn parse(parser: &mut Parser) -> () {
    let rules = define_operator(&mut parser.tree);
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];
    for mut reader in flatten_tree(parser.tree.clone()) {
        let rules = rules.clone();
        threads.push(thread::spawn(move || {
            parse_reader(rules, &mut reader);
        }));
    }

    for t in threads {
        t.join().unwrap()
    }
}

fn parse_reader(rules: Vec<SymbolRule>, reader: &mut Reader) {
    let (mut tokens, rules) = get_vec(rules.clone(), reader);

    process_symbol(rules, &mut tokens);
}

fn flatten_tree(tree: ReaderTree) -> Vec<Reader> {
    let v = vec![tree.now];
    let mut c: Vec<Vec<Reader>> = vec![];
    for tree in tree.readers {
        c.push(flatten_tree(tree));
    }

    [v, c.concat()].concat()
}

fn process_symbol(rules: Vec<SymbolRule>, tokens: &mut Vec<Token>) {
    let mut symbol_buf: Vec<String> = vec![];
    let mut seq = false;
    let mut start = 0;
    let mut len = 0;
    for (idx, token) in tokens.clone().iter().enumerate() {
        let idx = idx - len;
        match token.clone().ty {
            TokenType::Symbol(symbol) => {
                symbol_buf.push(symbol.clone());
                if !seq {
                    seq = true;
                    start = idx;
                }
            },
            _ => {
                if seq {
                    let end = idx;
                    let tokens_ = combine_symbol(rules.clone(), symbol_buf.concat(), tokens[start].position.start, tokens[end-1].position.end, tokens[end-1].position.line);
                    tokens.splice(start..end, tokens_);
                    len += end - start - 1;
                    symbol_buf.clear();
                    seq = false;
                } else {
                    continue;
                }
            }
        }
    }
}

fn combine_symbol(rules: Vec<SymbolRule>, symbol: String, start: u64, end: u64, line: u64) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    for rule in rules {
        let symbol_ = rule.symbols.iter().collect::<String>();
        if symbol.contains(symbol_.as_str()) {
            result.push(new_token(symbol_.clone(), TokenPosition {
                start: start + symbol.find(symbol_.as_str()).unwrap() as u64,
                end: end - symbol.find(symbol_.as_str()).unwrap() as u64,
                line
            }));
        }
    }

    result
}