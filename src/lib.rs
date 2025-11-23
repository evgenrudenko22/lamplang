use crate::translator::{ast::Stmt, lexer::{Token, Lexer}, parser::Parser, codegen::CCodeGenerator};

mod translator;

pub fn lex(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input.to_string());
    lexer.lex();
    lexer.tokens
}

pub fn parse_tokens(tokens: &Vec<Token>) -> Stmt {
    let mut parser = Parser::new(tokens.clone());
    parser.parse()
}

pub fn generate_c_code(stmt: Stmt) -> String {
    let mut generator = CCodeGenerator::new(stmt);
    let c_code = generator.generate();
    c_code
}

pub fn translate(input: &str) -> String {
    generate_c_code(parse_tokens(&lex(input)))
}