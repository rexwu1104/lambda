use std::fmt::Debug;

use colored::{ColoredString, Colorize};

use crate::lambda::reader::token::Token;

pub fn show_status(status: Status, token: Option<Token>) {
    get_mode(status.clone().ty)(status, token)
}

fn get_color(level: StatusLevel) -> fn(String) -> ColoredString {
    match level {
        StatusLevel::Error => |s| s.red().bold(),
        StatusLevel::Warning => |s| s.yellow().bold(),
        StatusLevel::Note => |s| s.bright_white().bold()
    }
}

fn get_mode(ty: StatusType) -> fn(Status, Option<Token>) -> () {
    match ty {
        StatusType::Lex => |s, t| {
            /**
             * Lex <status_level>: <status_message>
             * ---| path: <status_module_path>
             *    |
             * 12 | <token_raw>
             *    |
             */
            println!(
                "{}: {:?} {} {}: {:?}",
                "lexing token".bright_white().bold(),
                t.unwrap().ty,
                get_color(s.level)("-->".to_string()),
                get_color(s.level)(format!("{:?}", s.ty)),
                s.raw.clone()
            )
        },
        StatusType::Parse => |s, t| {},
        StatusType::Normal => |s, t| {},
        StatusType::File => |s, t| {}
    }
}

#[derive(Clone)]
pub struct Status {
    pub raw: String,
    pub module: String,
    pub ty: StatusType,
    pub level: StatusLevel,
    pub position: Option<StatusPosition>
}

#[derive(Clone)]
pub struct StatusPosition {
    pub start: u64,
    pub line: u64,
    pub end: Option<u64>
}

#[derive(Debug, Clone, Copy)]
pub enum StatusType {
    Normal,
    Lex,
    Parse,
    File
}

#[derive(Debug, Clone, Copy)]
pub enum StatusLevel {
    Error,
    Warning,
    Note
}