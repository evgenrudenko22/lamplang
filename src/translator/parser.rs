use super::ast::{Expr, TypedArgument, Stmt};
use super::lexer::{Lexeme, Token, TokenType};
use super::value::{Value, ValueType};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Stmt {
        let mut stmts: Vec<Stmt> = vec![];
        while !self.check(TokenType::Eof) {
            stmts.push(self.statement());
        }
        Stmt::Block(stmts)
    }

    fn parse_block(&mut self) -> Stmt {
        let mut stmts: Vec<Stmt> = vec![];
        self.consume(TokenType::LeftBrace);
        while !self.check(TokenType::RightBrace) {
            stmts.push(self.statement());
        }

        Stmt::Block(stmts)
    }

    fn statement_or_block(&mut self) -> Stmt {
        if self.get(0).token_type == TokenType::LeftBrace {
            return self.parse_block();
        }
        self.statement()
    }

    fn statement(&mut self) -> Stmt {
        if self.check(TokenType::If) {
            return self.if_else()
        }
        if self.check(TokenType::Var) {
            return self.var_def()
        }
        if self.check(TokenType::While) {
            return self.while_()
        }
        if self.get(0).token_type == TokenType::Word && self.get(1).token_type == TokenType::LeftParen {
            return Stmt::Function(Box::from(self.function()))
        }
        if self.check(TokenType::Func) {
            return self.function_define()
        }
        if self.check(TokenType::Return) {
            return Stmt::Return(Box::from(self.expression()))
        }
        if self.check(TokenType::Use) {
            return self.use_()
        }
        if self.check(TokenType::Struct) {
            return self.struct_()
        }

        self.assign()
    }

    fn struct_(&mut self) -> Stmt {
        let name = match self.consume(TokenType::Word).lexeme {
            Lexeme::WordLexeme(v) => v,
            _ => unreachable!(),
        };
        let fields: Vec<TypedArgument> = self.get_typed_arguments(TokenType::LeftBrace, TokenType::RightBrace);
        Stmt::Struct(name, fields)
    }

    fn use_(&mut self) -> Stmt {
        let module = match self.consume(TokenType::Word).lexeme {
            Lexeme::WordLexeme(v) => v,
            _ => unreachable!(),
        };
        Stmt::Use(module)
    }

    fn function_define(&mut self) -> Stmt {
        let name = match self.consume(TokenType::Word).lexeme {
            Lexeme::WordLexeme(v) => v,
            _ => unreachable!(),
        };
        let params = self.get_typed_arguments(TokenType::LeftParen, TokenType::RightParen);
        let return_value = self.parse_value_type();
        let body = self.statement_or_block();
        Stmt::FunctionDef(name, params, Box::from(body), return_value)
    }

    fn get_typed_arguments(&mut self, left: TokenType, right: TokenType) -> Vec<TypedArgument> {
        self.consume(left);
        let mut params: Vec<TypedArgument> = vec![];
        while !self.check(right) {
            let name = match self.consume(TokenType::Word).lexeme {
                Lexeme::WordLexeme(v) => v,
                _ => unreachable!(),
            };
            let value_type: ValueType = self.parse_value_type();
            params.push(TypedArgument {
                name,
                typ: value_type,
            });
            self.check(TokenType::Comma);
        }
        params
    }

    fn parse_value_type(&mut self) -> ValueType {
        self.consume(TokenType::Colon);
        let type_string = match self.consume(TokenType::Word).lexeme {
            Lexeme::WordLexeme(v) => v,
            _ => unreachable!()
        };
        let value_type: ValueType = match type_string.as_str() {
            "number" => ValueType::Number,
            "string" => ValueType::String,
            "unit" => ValueType::Unit,
            _ => panic!("incorrect value type")
        };
        value_type
    }

    fn while_(&mut self) -> Stmt {
        let cond = self.expression();
        let body = self.statement_or_block();
        Stmt::While(Box::from(cond), Box::from(body))
    }

    fn if_else(&mut self) -> Stmt {
        let cond = self.expression();
        let body = self.statement_or_block();
        let mut else_body: Option<Stmt> = None;
        if self.check(TokenType::Else) {
            else_body = Some(self.statement_or_block());
        }

        Stmt::If(Box::from(cond), Box::from(body), Box::from(else_body))
    }

    fn assign(&mut self) -> Stmt {
        let cur = self.get(0);
        if self.check(TokenType::Word) && self.get(0).token_type == TokenType::Eq {
            let name = match cur.lexeme {
                Lexeme::WordLexeme(v) => v,
                _ => unreachable!()
            };
            self.consume(TokenType::Eq);
            return Stmt::Assign(name, Box::from(self.expression()));
        }
        panic!("Invalid assignment at {:#?}", self.pos..self.pos);
    }

    fn var_def(&mut self) -> Stmt {
        let cur = self.get(0);
        if self.check(TokenType::Word) && self.get(0).token_type == TokenType::Colon
            && self.get(1).token_type == TokenType::Word && self.get(2).token_type == TokenType::Eq {
            let name = match cur.lexeme {
                Lexeme::WordLexeme(v) => v,
                _ => unreachable!()
            };
            let value_type = self.parse_value_type();
            self.consume(TokenType::Eq);

            return Stmt::VarDef(name, Box::new(self.expression()), value_type);
        }
        panic!("Invalid var definition at {:?}", cur.start..cur.end);
    }

    fn function(&mut self) -> Expr {
        let name = match self.consume(TokenType::Word).lexeme {
            Lexeme::WordLexeme(v) => v,
            _ => unreachable!()
        };
        self.consume(TokenType::LeftParen);
        let mut params: Vec<Expr> = vec![];
        while !self.check(TokenType::RightParen) {
            params.push(self.expression());
            self.check(TokenType::Comma);
        }

        Expr::Functional(name, params)
    }

    fn expression(&mut self) -> Expr {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Expr {
        let mut result = self.logical_and();

        loop {
            if self.check(TokenType::OrOr) {
                result = Expr::Condition("||".to_string(), Box::new(result), Box::new(self.logical_and()));
                continue
            }
            break
        }

        result
    }

    fn logical_and(&mut self) -> Expr {
        let mut result = self.equal();

        loop {
            if self.check(TokenType::AndAnd) {
                result = Expr::Condition("&&".to_string(), Box::from(result), Box::from(self.equal()));
                continue
            }
            break
        }

        result
    }

    fn equal(&mut self) -> Expr {
        let mut result = self.compare();

        loop {
            if self.check(TokenType::EqEq) {
                result = Expr::Condition("==".to_string(), Box::from(result), Box::from(self.compare()));
                continue
            } else if self.check(TokenType::NoEq) {
                result = Expr::Condition("!=".to_string(), Box::from(result), Box::from(self.compare()));
                continue
            }
            break
        }


        result
    }

    fn compare(&mut self) -> Expr {
        let mut result = self.multiply();

        loop {
            if self.check(TokenType::Gt) {
                result = Expr::Condition(">".to_string(), Box::from(result), Box::from(self.multiply()));
                continue
            } else if self.check(TokenType::Lt) {
                result = Expr::Condition("<".to_string(), Box::from(result), Box::from(self.multiply()));
                continue
            } else if self.check(TokenType::GtEq) {
                result = Expr::Condition(">=".to_string(), Box::from(result), Box::from(self.multiply()));
                continue
            } else if self.check(TokenType::LtEq) {
                result = Expr::Condition("<=".to_string(), Box::from(result), Box::from(self.multiply()));
                continue
            }
            break
        }

        result
    }

    fn multiply(&mut self) -> Expr {
        let mut result = self.addition();

        loop {
            if self.check(TokenType::Star) {
                result = Expr::Binary('*', Box::from(result), Box::from(self.addition()));
                continue
            } else if self.check(TokenType::Slash) {
                result = Expr::Binary('/', Box::from(result), Box::from(self.addition()));
                continue
            }
            break
        }

        result
    }

    fn addition(&mut self) -> Expr {
        let mut result = self.unary();

        loop {
            if self.check(TokenType::Plus) {
                result = Expr::Binary('+', Box::from(result), Box::from(self.unary()));
                continue
            } else if self.check(TokenType::Minus) {
                result = Expr::Binary('-', Box::from(result), Box::from(self.unary()));
                continue
            }
            break
        }

        result
    }


    fn unary(&mut self) -> Expr {
        if self.check(TokenType::Minus) {
            return Expr::Unary('-', Box::from(self.primary()));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        let token = self.get(0);
        if self.check(TokenType::Number) {
            if let Lexeme::NumberLexeme(v) = token.lexeme {
                return Expr::Value(Value::Number(v))
            }
        }
        if self.check(TokenType::String) {
            if let Lexeme::StringLexeme(v) = token.lexeme {
                return Expr::Value(Value::String(v))
            }
        }
        if self.get(0).token_type == TokenType::New && self.get(1).token_type == TokenType::Word {
            self.consume(TokenType::New);
            let name = match self.consume(TokenType::Word).lexeme {
                Lexeme::WordLexeme(v) => v,
                _ => unreachable!(),
            };
            let params = self.get_typed_arguments(TokenType::LeftParen, TokenType::RightParen);
            return Expr::New(name, params)
        }
        if self.get(0).token_type == TokenType::Word && self.get(1).token_type == TokenType::LeftParen {
            return self.function()
        }
        if self.check(TokenType::Word) {
            if let Lexeme::WordLexeme(v) = token.lexeme {
                return Expr::VarUse(v)
            }
        }
        if self.check(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen);
            return expr;
        }
        panic!("Invalid primary: {:#?}", token);
    }

    fn consume(&mut self, token_type: TokenType) -> Token {
        let result = self.tokens.get(self.pos).unwrap();
        assert_eq!(result.token_type, token_type, "expected token type {:?}, but founded {:?} at {:?}", token_type, result.token_type, result.start..result.end);
        self.pos += 1;
        result.clone()
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        let result = self.get(0).token_type == token_type;
        if result {
            self.pos += 1;
        }
        result
    }

    fn get(&mut self, relative_pos: usize) -> Token {
        let position = self.pos + relative_pos;
        self.tokens.get(position).unwrap().clone()
    }
}