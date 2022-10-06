use regex::Regex;

use super::{this::{Reader, ReaderTree, create_reader, SymbolRule}, token::Token};

#[inline]
fn import(reader: &mut Reader) -> Vec<String> {
    let import_r = Regex::new("(?P<key>formula[ ]*)\"(?P<path>.*)\"").unwrap();
    let all_path: Vec<(usize, usize, String)> =
        import_r.captures_iter(reader.raw.as_str()).map(|c| {
            let m = c.name("path").unwrap();
            (c.name("key").unwrap().start(), m.end(), m.as_str().to_string())
        }).collect();
    
    all_path.iter().map(|t| {
        reader.raw = reader.raw.replace(&reader.raw[t.0..=t.1], "");
        t.2.clone()
    }).rev().collect()
}

pub fn define_operator(tree: &mut ReaderTree) -> Vec<SymbolRule> {
    let operator_rn = Regex::new(r"\{(?P<rule>[^\{^\}]*)\}\s*\((?P<symbols>[^\(^\)]*)\)\s*of\s*(?P<variables>(?:[a-zA-Z]+\s*)*)\s*=\s*(?P<body>(?:[^e]|(?:e)(?:[n][^d]|[^n][^d]|$)\b)*)end\b").unwrap();
    let operator_r = Regex::new(r"\{(?P<rule>[^\{^\}]*)\}\s*\((?P<symbols>[^\(^\)]*)\)\s*of\s*(?P<variables>(?:[a-zA-Z]+\s*)*)\s*=\s*(?P<body>[^\n]*)").unwrap();
    let mut result: Vec<SymbolRule> = vec![];
    if !tree.readers.is_empty() {
        for symbols in tree.readers.iter_mut().map(|t| define_operator(t)) {
            result = [result, symbols].concat();
        }
    }

    for cap in operator_rn.captures_iter(tree.now.raw.to_owned().as_str()) {
        tree.now.raw = tree.now.raw.replace(&cap[0], "");
        result.push(SymbolRule{
            raw: cap["rule"].to_string(),
            symbols: cap["symbols"].chars().collect(),
            variables: cap["variables"].trim().split(' ').map(String::from).collect(),
            body: cap["body"].trim().to_string()
        });
    }

    for cap in operator_r.captures_iter(tree.now.raw.to_owned().as_str()) {
        tree.now.raw = tree.now.raw.replace(&cap[0], "");
        result.push(SymbolRule{
            raw: cap["rule"].to_string(),
            symbols: cap["symbols"].replace(' ', "").chars().collect(),
            variables: cap["variables"].trim().split(' ').map(String::from).collect(),
            body: cap["body"].trim().to_string()
        });
    }

    result
}

fn pre_process(reader: &mut Reader) -> ReaderTree {
    let mut module_num: u64 = 0;
    let readers: Vec<Reader> =
        import(reader).iter().map(|p| {
            module_num+=1; create_reader(
                p.clone(),
                format!("{}_{}", reader.module, module_num),
                Some(reader.file.current_dir.clone())
            )
        }).collect();

    reader.raw = reader.raw.replace("\\\r\n", "");
    ReaderTree{
        readers: readers.iter().map(|r| pre_process(&mut r.clone())).collect(),
        now: reader.to_owned(),
    }
}

#[inline]
pub fn get_vec(rules: Vec<SymbolRule>, reader: &mut Reader) -> (Vec<Token>, Vec<SymbolRule>) {
    reader.to_owned().into_vec(rules)
}

#[inline]
pub fn lex(path: String) -> ReaderTree {
    pre_process(&mut create_reader(path, "__main".to_lowercase(), None))
}