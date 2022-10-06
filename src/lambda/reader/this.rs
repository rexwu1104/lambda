use super::file::{File, configure_file, FileType, read_file};

#[inline]
pub fn create_reader(file: String, module: String, home: Option<String>) -> Reader {
    let file = configure_file(file.clone(), home);

    if file.file_type == FileType::UnkownCode {
        panic!("can't compile unknown code");
    }

    Reader {
        file: file.clone(),
        module,
        raw: read_file(file)
    }
}

#[inline]
pub fn create_system_rule(symbol: &str, variables: Vec<String>) -> SymbolRule {
    SymbolRule {
        raw: String::new(),
        symbols: symbol.chars().collect(),
        variables,
        body: String::from("builtin")
    }
}

#[derive(Debug, Clone)]
pub struct Reader {
    pub file: File,
    pub module: String,
    pub raw: String
}

#[derive(Debug, Clone)]
pub struct ReaderTree {
    pub readers: Vec<ReaderTree>,
    pub now: Reader
}

#[derive(Debug, Clone)]
pub struct SymbolRule {
    pub raw: String,
    pub symbols: Vec<char>,
    pub variables: Vec<String>,
    pub body: String
}