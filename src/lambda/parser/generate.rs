use crate::lambda::reader::token::{Token, TokenType, TokenPosition};
use super::{iter::StatementIter, tree::{Statement, StatementContent, Function, Type, Decorator, Body, Value, Expression, Variable, ExpressionContent, CallFunction, TypeContent, Class, Tuple, ValueType, Struct}};

pub fn generate_statement(iter: &mut StatementIter) -> Option<Statement> {
    skip_space(iter);

    let mut decorators: Vec<Decorator> = vec![];
    while get_token(iter).raw == "@" {
        decorators.push(generate_decorator(iter));

        forward_iter(iter);
    }

    if get_token(iter).ty == TokenType::Newline {
        skip_space(iter);
    }

    let content: Option<StatementContent>;
    if get_token(iter).raw == "type" { // define type
        forward_iter(iter);
        let name = get_token(iter).raw;
        content = Some(StatementContent::Type(generate_define_type(iter, name, decorators)));
        println!("{:#?}", content);
    } else {
        let name = get_token(iter).raw;
        forward_iter(iter);
        match get_token(iter).raw.as_str() {
            "of" => {
                content = Some(StatementContent::Function(generate_define_function(iter, name, decorators)));
                println!("{:#?}", content);
            }, // define function
            "=" => {
                content = Some(StatementContent::Variable(generate_define_variable(iter, name, decorators)));
                println!("{:#?}", content);
            }, // define global variable
            _ => {
                if get_token(iter).ty == TokenType::Newline {
                    content = Some(StatementContent::Variable(generate_define_variable(iter, name, decorators)));
                    println!("{:#?}", content);
                } else {
                    content = None
                }
            } // other situation
        }
    }

    if content != None {
        Some(Statement {
            content: content.unwrap()
        })
    } else {
        None
    }
}

fn generate_define_function(iter: &mut StatementIter, name: String, decorators: Vec<Decorator>) -> Function {
    forward_iter(iter);
    let mut params: Vec<String> = vec![];
    while get_token(iter).raw != "=" {
        if get_token(iter).raw == "()" {
            forward_iter(iter);
            break;
        }

        params.push(get_token(iter).raw);
        forward_iter(iter);
    }

    forward_iter(iter);
    if get_token(iter).ty != TokenType::Newline {
        Function {
            decorators,
            name,
            params,
            body: generate_define_function_body(iter, false)
        }
    } else {
        forward_iter(iter);
        Function {
            decorators,
            name,
            params,
            body: generate_define_function_body(iter, true)
        }
    }
}

fn generate_decorator(iter: &mut StatementIter) -> Decorator {
    forward_iter(iter);
    Decorator {
        tag: get_token(iter).raw
    }
}

fn generate_define_function_body(iter: &mut StatementIter, newline: bool) -> Body {
    let mut expressions: Vec<Expression> = vec![];
    skip_space(iter);
    if newline {
        while get_token(iter).raw != "end" {
            expressions.push(generate_expression(iter));

            forward_iter(iter);
        }
    } else {
        expressions.push(generate_expression(iter))
    }

    Body {
        expressions
    }
}

fn generate_expression(iter: &mut StatementIter) -> Expression {
    let mut decorators: Vec<Decorator> = vec![];
    while get_token(iter).raw == "@" {
        decorators.push(generate_decorator(iter));

        forward_iter(iter);
    }

    let name = get_token(iter).raw;
    forward_iter(iter);
    if get_token(iter).raw == "of" {
        Expression {
            content: ExpressionContent::DefineFunction(generate_define_function(iter, name, decorators))
        }
    } else if get_token(iter).ty == TokenType::Newline || get_token(iter).raw == "=" {
        Expression {
            content: ExpressionContent::DefineVariable(generate_define_variable(iter, name, decorators))
        }
    } else {
        Expression {
            content: ExpressionContent::CallFunction(generate_call_function(iter, Value::Identifier(Box::new(name))))
        }
    }
}

fn generate_define_variable(iter: &mut StatementIter, name: String, decorators: Vec<Decorator>) -> Variable {
    if get_token(iter).ty == TokenType::Newline {
        Variable {
            decorators,
            name,
            value: None
        }
    } else {
        forward_iter(iter);
        Variable {
            decorators,
            name,
            value: Some(generate_value(iter))
        }
    }
}

fn generate_value(iter: &mut StatementIter) -> Value { // not complete
    fn match_token(token: Token) -> Value {
        match token.ty {
            TokenType::Atom(a) => Value::Identifier(Box::new(format!("Atom:{}", a))),
            TokenType::Boolean(b) => Value::Boolean(Box::new(b)),
            TokenType::Char(c) => Value::Char(Box::new(c)),
            TokenType::Float(f) => Value::Float(Box::new(f)),
            TokenType::Identifier(i) => Value::Identifier(Box::new(i)),
            TokenType::Number(n) => if n.starts_with('-') {
                Value::Integer(Box::new(n.parse().unwrap()))
            } else {
                Value::Unsigned(Box::new(n.parse().unwrap()))
            },
            TokenType::String(s) => Value::String(Box::new(s)),
            TokenType::Symbol(s) => if s == "?" {
                Value::MeaningLess
            } else {
                Value::Identifier(Box::new(s))
            },
            _ => Value::NotAValue
        }
    }

    let token = get_token(iter);
    forward_iter(iter);
    if get_token(iter).ty == TokenType::Newline {
        match_token(token)
    } else {
        Value::Call(Box::new(generate_call_function(iter, match_token(token))))
    }
}

fn generate_call_function(iter: &mut StatementIter, name: Value) -> CallFunction {
    CallFunction {
        name,
        value: generate_value(iter)
    }
}

fn generate_define_type(iter: &mut StatementIter, name: String, decorators: Vec<Decorator>) -> Type {
    let ty: TypeContent;
    forward_iter(iter);
    if get_token(iter).raw == "(" {
        ty = generate_type_content_tuple_or_struct(iter)
    } else {
        ty = TypeContent::TupleType(Tuple {
            variables: vec![]
        })
    }

    Type {
        decorators,
        name,
        ty
    }
}

fn generate_type_content_tuple_or_struct(iter: &mut StatementIter) -> TypeContent {
    forward_iter(iter);
    if get_token(iter).raw == ")" {
        TypeContent::TupleType(Tuple {
            variables: vec![]
        })
    } else {
        let name_or_type = get_token(iter).raw;
        forward_iter(iter);
        if get_token(iter).raw == ":" {
            generate_type_content_struct(iter, name_or_type)
        } else if get_token(iter).raw == ")" {
            TypeContent::TupleType(Tuple {
                variables: vec![ValueType {
                    name: name_or_type
                }]
            })
        } else {
            generate_type_content_tuple(iter, name_or_type)
        }
    }
}

fn generate_type_content_tuple(iter: &mut StatementIter, first: String) -> TypeContent {
    let mut variables = vec![ValueType {
        name: first
    }, ValueType {
        name: get_token(iter).raw
    }];

    forward_iter(iter);
    while get_token(iter).raw != ")" {
        variables.push(ValueType {
            name: get_token(iter).raw
        });

        forward_iter(iter)
    }

    TypeContent::TupleType(Tuple {
        variables
    })
}

fn generate_type_content_struct(iter: &mut StatementIter, first: String) -> TypeContent {
    forward_iter(iter);
    let mut variables = vec![(first, ValueType {
        name: get_token(iter).raw
    })];

    forward_iter(iter);
    while get_token(iter).raw != ")" {
        let name = get_token(iter).raw;
        forward_iter(iter);
        if get_token(iter).raw != ":" {

        }

        forward_iter(iter);
        variables.push((name, ValueType {
            name: get_token(iter).raw
        }));

        forward_iter(iter)
    }

    TypeContent::StructType(Struct {
        variables
    })
}

// fn contains_token(iter: &mut StatementIter, f: fn(&Token) -> bool) -> bool {
//     iter.tokens.iter().any(f)
// }

// fn find_token(iter: &mut StatementIter, f: fn(&Token) -> bool) -> Option<usize> {
//     iter.tokens.iter().position(f)
// }

fn get_token(iter: &mut StatementIter) -> Token {
    if iter.idx < iter.tokens.len() {
        iter.tokens[iter.idx].clone()
    } else {
        Token {
            ty: TokenType::End,
            raw: "".into(),
            position: TokenPosition {
                start: 0,
                end: 0,
                line: 0
            }
        }
    }
}

fn forward_iter(iter: &mut StatementIter) -> () {
    iter.idx += 1
}

// fn backward_iter(iter: &mut StatementIter) -> () {
//     iter.idx -= 1
// }

fn skip_space(iter: &mut StatementIter) -> () {
    let mut token = get_token(iter);
    while token.ty == TokenType::Newline || token.ty == TokenType::Comment {
        forward_iter(iter);

        token = get_token(iter);
    }
}
