use super::value::{Value, ValueType};

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    VarUse(String),
    Binary(char, Box<Expr>, Box<Expr>),
    Condition(String, Box<Expr>, Box<Expr>),
    Unary(char, Box<Expr>),
    Functional(String, Vec<Expr>),
    New(String, Vec<TypedArgument>),
}

#[derive(Debug, Clone)]
pub struct TypedArgument {
    pub name: String,
    pub typ: ValueType,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDef(String, Box<Expr>, ValueType),
    Assign(String, Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Box<Option<Stmt>>),
    Block(Vec<Stmt>),
    While(Box<Expr>, Box<Stmt>),
    Function(Box<Expr>),
    FunctionDef(String, Vec<TypedArgument>, Box<Stmt>, ValueType),
    Return(Box<Expr>),
    Use(String),
    Struct(String, Vec<TypedArgument>),
}