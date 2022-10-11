use std::cmp::Ordering;

use regex::Regex;

use crate::lambda::reader::methods::remove_surplus;

use super::{token::{TokenIterator, Token, new_token, TokenPosition}, this::{Reader, create_system_rule, SymbolRule}};

impl TokenIterator {
    pub fn load_system_symbol(&mut self) -> TokenIterator {
        self.rules.push(create_system_rule("->", "$x->$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("()", "()".to_string(), vec![]));
        self.rules.push(create_system_rule("(", "($x".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule(")", "$x)".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule(":", "$x:$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("=>", "$x=>$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("[", "[$x".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule("]", "$x]".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule("[]", "[]$x".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule(".", "$x.$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("|>", "$x|>$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("?", "?".to_string(), vec![]));
        self.rules.push(create_system_rule("@", "@$x".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule("$", "$x$$$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule(";", "$x;$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("_", "$x_$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("<<", "$x<<$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule(">>", "$x>>$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("|", "$x|$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("&", "$x&$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("^", "$x^$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("~", "~$x".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule("**", "$x**$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("*", "$x*$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("/", "$x/$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("..", "$x..$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("..=", "$x..=$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("+", "$x+$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("-", "$x-$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("-", "-$x".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule("%", "$x%$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("++", "$x++$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule(">", "$x>$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("<", "$x<$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule(">=", "$x>=$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("<=", "$x<=$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("==", "$x==$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("!=", "$x!=$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("!", "!$x".to_string(), vec!["x".into()]));
        self.rules.push(create_system_rule("||", "$x||$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("&&", "$x&&$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule(",", "$x,$y".to_string(), vec!["x".into(), "y".into()]));
        self.rules.push(create_system_rule("=", "$x=$y".to_string(), vec!["x".into(), "y".into()]));
        self.to_owned()
    }
    
    #[inline]
    pub fn load_other_rules(&mut self, rules: Vec<SymbolRule>) -> TokenIterator {
        self.rules = [self.rules.clone(), rules].concat();

        self.to_owned()
    }

    #[inline]
    pub fn sort(&mut self) -> TokenIterator {
        self.rules.sort_by(|a, b| {
            let ord = a.symbols.len().cmp(&b.symbols.len());
            if ord == Ordering::Equal {
                if a.symbols.cmp(&b.symbols) == Ordering::Less { Ordering::Greater } else { Ordering::Less }
            } else {
                if ord == Ordering::Less { Ordering::Greater } else { Ordering::Less }
            }
        });

        self.to_owned()
    }
}

impl TokenIterator {
    fn tokenize(&mut self) -> Vec<Token> {
        let mut result = [
            self.lex_number(),
            self.lex_atom(),
            self.lex_string(),
            self.lex_char(),
            self.lex_identifier(),
            self.lex_symbol(),
            self.lex_space(),
            self.lex_comment()
        ].concat();
        result.sort_by(|a, b| {
            a.position.start.cmp(&b.position.start)
        });

        result = remove_surplus(result);
        for t in result.iter().rev() {
            self.raw.replace_range(t.position.start as usize..t.position.end as usize, "");
        }

        if !self.raw.trim().is_empty() {
            // unknown symbol
        }

        result
    }

    fn lex_identifier(&self) -> Vec<Token> {
        let identifier_r = Regex::new(r"(?P<name>[a-zA-Z][a-zA-Z0-9]*)").unwrap();

        let mut tokens: Vec<Token> = vec![];
        for cap in identifier_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("name").unwrap();
            tokens.push(
                new_token(cap["name"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }

    fn lex_number(&self) -> Vec<Token> {
        let float_r = Regex::new(r"(?P<num>(?:[0-9]+\.[0-9]*|[0-9]*\.[0-9]+))").unwrap();
        let integer_r = Regex::new(r"(?P<num>[0-9]+)").unwrap();

        let mut tokens: Vec<Token> = vec![];
        for cap in float_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("num").unwrap();
            tokens.push(
                new_token(cap["num"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }
        
        for cap in integer_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("num").unwrap();
            tokens.push(
                new_token(cap["num"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }

    fn lex_string(&self) -> Vec<Token> {
        let raw_r = Regex::new(r#"(?P<str>#(?:[^\\#]|\\#)*#)"#).unwrap();
        let str_r = Regex::new(r#"(?P<str>"(?:[^\\"\n]|\\")*")"#).unwrap();

        let mut tokens: Vec<Token> = vec![];
        for cap in raw_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("str").unwrap();
            tokens.push(
                new_token(cap["str"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }
        
        for cap in str_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("str").unwrap();
            tokens.push(
                new_token(cap["str"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }
    
    fn lex_char(&self) -> Vec<Token> {
        let char_r = Regex::new(r"(?P<chr>'(?:[^\\']|\\')*')").unwrap();

        let mut tokens: Vec<Token> = vec![];
        for cap in char_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("chr").unwrap();
            tokens.push(
                new_token(cap["chr"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }
    
    fn lex_symbol(&self) -> Vec<Token> {
        let symbol_r = Regex::new(r#"(?P<single>[~!@#\$%\^&\*\(\)_\-\+=\{\}\[\]\|\\:";'<>\?,\./])"#).unwrap();
        
        let mut tokens: Vec<Token> = vec![];
        for cap in symbol_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("single").unwrap();
            tokens.push(
                new_token(cap["single"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }

    fn lex_space(&self) -> Vec<Token> {
        let space_r = Regex::new(r#"(?P<space>\s+)"#).unwrap();
        
        let mut tokens: Vec<Token> = vec![];
        for cap in space_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("space").unwrap();
            tokens.push(
                new_token(cap["space"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }

    fn lex_atom(&self) -> Vec<Token> {
        let atom_r = Regex::new(r"(?P<atom>`(?:[^\\`]|\\`)*`)").unwrap();
        
        let mut tokens: Vec<Token> = vec![];
        for cap in atom_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("atom").unwrap();
            tokens.push(
                new_token(cap["atom"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }

    fn lex_comment(&self) -> Vec<Token> {
        let atom_r = Regex::new(r"(?P<comment>//[^\r]*)").unwrap();
        
        let mut tokens: Vec<Token> = vec![];
        for cap in atom_r.captures_iter(self.raw.as_str()) {
            let pos = cap.name("comment").unwrap();
            tokens.push(
                new_token(cap["comment"].into(), TokenPosition {
                    start: pos.start() as u64,
                    end: pos.end() as u64,
                    line: self.raw[0..pos.start()].matches("\r").count() as u64
                })
            )
        }

        tokens.iter().rev().map(|t| t.to_owned()).collect()
    }
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if let Some(t) = &self.token {
            Some(t.to_owned())
        } else {
            None
        }
    }
}

impl Reader {
    pub fn into_vec(self, rules: Vec<SymbolRule>) -> (Vec<Token>, Vec<SymbolRule>) {
        let mut iter = TokenIterator {
            token: None,
            index: 0,
            raw: self.raw,
            rules: vec![]
        }.load_system_symbol().load_other_rules(rules).sort();

        (iter.tokenize(), iter.rules)
    }
}
