use std::time::Instant;

use lambda::reader::methods::lex;

use crate::lambda::parser::{this::new_parser, methods::parse};

pub mod lambda;

// use lambda::{utils::status::{show_status, Status, StatusType, StatusLevel}, reader::token::{Token, TokenType, TokenPosition}};

fn main() -> () {
    let start = Instant::now();
    parse(&mut new_parser(lex("./test/main.ld".to_string())));
    println!("{} milliseconds", (Instant::now() - start).as_millis());
}

// show_status(Status{
//     raw: "\"ssss".to_string(),
//     ty: StatusType::Lex,
//     level: StatusLevel::Error,
//     module: "__main".to_string(),
//     position: None
// }, Some(Token{
//     ty: TokenType::String("ssss".to_string()),
//     raw: String::new(),
//     position: TokenPosition{
//         start: 0,
//         end: 0,
//         line: 0
//     }
// }));