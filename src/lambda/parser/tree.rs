#[derive(Debug, PartialEq)]
pub struct Statement {
    pub content: StatementContent
}

#[derive(Debug, PartialEq)]
pub struct Decorator {
    pub tag: String
}

#[derive(Debug, PartialEq)]
pub enum StatementContent {
    Function(Function),
    Type(Type),
    Variable(Variable)
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub decorators: Vec<Decorator>,
    pub name: String,
    pub params: Vec<String>,
    pub body: Body
}

#[derive(Debug, PartialEq)]
pub struct Body {
    pub expressions: Vec<Expression>
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub content: ExpressionContent
}

#[derive(Debug, PartialEq)]
pub enum ExpressionContent {
    CallFunction(CallFunction),
    DefineFunction(Function),
    DefineVariable(Variable)
}

#[derive(Debug, PartialEq)]
pub struct CallFunction {
    pub name: Value,
    pub value: Value
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(Box<String>),
    Char(Box<char>),
    Float(Box<f64>),
    Integer(Box<i128>),
    Unsigned(Box<u128>),
    Boolean(Box<bool>),
    Identifier(Box<String>),
    Call(Box<CallFunction>),
    MeaningLess,
    NotAValue
}

#[derive(Debug, PartialEq)]
pub struct Type {
    pub decorators: Vec<Decorator>,
    pub name: String,
    pub ty: TypeContent
}

#[derive(Debug, PartialEq)]
pub enum TypeContent {
    ClassType(Class),
    StructType(Struct),
    TupleType(Tuple)
}

#[derive(Debug, PartialEq)]
pub struct Class {
    pub methods: Vec<Function>,
    pub variables: Vec<Variable>
}

#[derive(Debug, PartialEq)]
pub struct Struct {
    pub variables: Vec<(String, ValueType)>
}

#[derive(Debug, PartialEq)]
pub struct ValueType {
    pub name: String
}

#[derive(Debug, PartialEq)]
pub struct Tuple {
    pub variables: Vec<ValueType>
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub decorators: Vec<Decorator>,
    pub name: String,
    pub value: Option<Value>
}