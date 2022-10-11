use std::thread;

use crate::lambda::reader::{methods::{get_vec, define_operator}, this::{Reader, ReaderTree, SymbolRule}, token::{Token, TokenType, new_token, TokenPosition}};

use super::{this::{Parser, new_iter}, tree::Statement, iter::StatementIter};

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

fn parse_reader(rules: Vec<SymbolRule>, reader: &mut Reader) -> Vec<Statement> {
    let (mut tokens, rules) = get_vec(rules.clone(), reader);

    tokens = process_symbol(rules.clone(), tokens);
    generate_parse_tree(rules, remove_space(tokens))
}

fn generate_parse_tree(rules: Vec<SymbolRule>, tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements: Vec<Statement> = vec![];
    let statement_iter: StatementIter = new_iter(rules, tokens.clone());
    for statement in statement_iter {
        statements.push(statement);
    }

    // println!("{:#?}", tokens.clone());

    statements
}

fn flatten_tree(tree: ReaderTree) -> Vec<Reader> {
    let v = vec![tree.now];
    let mut c: Vec<Vec<Reader>> = vec![];
    for tree in tree.readers {
        c.push(flatten_tree(tree));
    }

    [v, c.concat()].concat()
}

fn process_symbol(rules: Vec<SymbolRule>, tokens: Vec<Token>) -> Vec<Token> {
    let mut tokens = tokens.clone();
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
                    let tokens_ = combine_symbol(rules.clone(), symbol_buf.concat(), tokens[start].position.start, tokens[end-1].position.line);
                    
                    symbol_buf.clear();
                    tokens.splice(start..end, tokens_.clone());

                    seq = false;
                    len += end - start - tokens_.len();
                } else {
                    continue;
                }
            }
        }
    }

    if seq {
        let end = tokens.len();
        let tokens_ = combine_symbol(rules.clone(), symbol_buf.concat(), tokens[start].position.start, tokens[end-1].position.line);
        
        symbol_buf.clear();
        tokens.splice(start..end, tokens_.clone());
    }

    tokens
}

fn combine_symbol(rules: Vec<SymbolRule>, symbol: String, pos: u64, line: u64) -> Vec<Token> {
    let mut result: Vec<Token> = vec![];
    let mut positions: Vec<(String, usize)> = vec![];
    for rule in rules {
        let symbol_ = rule.symbols.iter().collect::<String>();
        for (idx, _) in symbol.match_indices(&symbol_) {
            positions.push((symbol_.clone(), idx));
        }
    }

    for (symbol, p) in positions {
        result.push(new_token(symbol.clone(), TokenPosition {
            start: pos + p as u64,
            end: (pos as usize + p + symbol.len()) as u64,
            line
        }));
    }

    if result.len() == 0 {
        return result;
    }

    let mut del_buf: Vec<usize> = vec![];
    let mut prev = &result[0];
    for (idx, t) in result.iter().enumerate() {
        if idx == 0 { continue; }
        if t.position.start >= prev.position.start &&
           t.position.end <= t.position.end {
            del_buf.push(idx);
        } else {
            prev = t;
        }
    }

    del_buf.iter().rev().for_each(|&i| { result.remove(i); });

    result
}

fn remove_space(tokens: Vec<Token>) -> Vec<Token> {
    tokens.iter().filter(|&t| {
        t.raw.replace(" ", "").len() != 0
    }).map(|t| {
        if t.ty == TokenType::Space {
            Token {
                raw: t.raw.to_owned(),
                ty: TokenType::Newline,
                position: t.position.to_owned()
            }
        } else {
            t.to_owned()
        }
    }).collect()
}