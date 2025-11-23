use std::collections::HashMap;

pub struct Lexer {
    input: String,
    pos: usize,
    pub tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenType>,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Lexemes
    String,
    Number,
    Word,

    // Symbols
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Eq,
    NoEq,
    EqEq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    AndAnd,
    OrOr,
    Comma,
    Colon,

    // Keywords
    Var,
    If,
    Else,
    Func,
    While,
    Return,
    Use,
    Struct,
    New,

    Eof
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    NumberLexeme(f32),
    StringLexeme(String),
    WordLexeme(String),
    None
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Lexeme,
    pub start: usize,
    pub end: usize,
}

static OPERATORS: &str = "+-*/=<>(){}!&|,:";

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Self {
            input,
            pos: 0,
            tokens: Vec::new(),
            keywords: Self::create_keywords(),
        }
    }

    fn create_keywords() -> HashMap<&'static str, TokenType> {
        HashMap::from([
            ("var", TokenType::Var),
            ("if", TokenType::If),
            ("else", TokenType::Else),
            ("func", TokenType::Func),
            ("while", TokenType::While),
            ("return", TokenType::Return),
            ("use", TokenType::Use),
            ("struct", TokenType::Struct),
            ("new", TokenType::New),
        ])
    }

    pub fn lex(&mut self) {
        loop {
            let next = match self.peek() {
                Some(next) => next,
                _ => break,
            };
            if next.is_digit(10) {
                self.lex_digit();
                continue;
            }
            if next == '"' {
                self.pos += 1;
                self.lex_string();
                continue;
            }
            if OPERATORS.contains(next) {
                self.lex_operators();
                continue;
            }
            if next.is_ascii_alphabetic() {
                self.lex_word();
                continue;
            }
            if next.is_whitespace() {
                self.pos += 1;
                continue;
            }
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: Lexeme::None,
            start: self.pos,
            end: self.pos,
        });
        println!("{:#?}", self.tokens);
    }

    fn lex_digit(&mut self) {
        let start: usize = self.pos;

        while self.peek().unwrap_or_default().is_digit(10) {
            self.pos += 1;
        }

        let number = self.input[start..self.pos].parse::<i32>().unwrap();

        self.tokens.push(Token {
            token_type: TokenType::Number,
            lexeme: Lexeme::NumberLexeme(number as f32),
            start,
            end: self.pos
        });
    }

    fn lex_string(&mut self) {
        let start: usize = self.pos;

        loop {
            let peek = self.peek().unwrap_or_default();
            if !self.check_string(peek) {
                break;
            }
            self.pos += 1;
        }

        let string = self.input[start..self.pos].to_string();
        self.tokens.push(Token {
            token_type: TokenType::String,
            lexeme: Lexeme::StringLexeme(string),
            start: start - 1,
            end: self.pos
        });

        self.pos += 1;
    }

    fn check_string(&mut self, symbol: char) -> bool {
        symbol != '"' && (symbol != '\n' || symbol != '\0')
    }

    fn lex_operators(&mut self) {
        use TokenType::*;
        let start: usize = self.pos;

        let mut str: std::string::String = std::string::String::new();

        let mut token_type: TokenType = Plus;
        let mut is_founded = false;

        while OPERATORS.contains(self.peek().unwrap_or_default()) {
            let symbol = self.peek().unwrap_or_default();
            str += &symbol.to_string();
            self.pos += 1;
            is_founded = true;
            token_type = match str.as_str() {
                "+" => Plus,
                "-" => Minus,
                "*" => Star,
                "/" => Slash,
                "!=" => NoEq,
                "==" => EqEq,
                ">" => Gt,
                ">=" => GtEq,
                "<" => Lt,
                "<=" => LtEq,
                "(" => LeftParen,
                ")" => RightParen,
                "{" => LeftBrace,
                "}" => RightBrace,
                "&&" => AndAnd,
                "||" => OrOr,
                "," => Comma,
                ":" => Colon,
                _ => {
                    is_founded = false;
                    continue
                },
            };
            break;
        }

        if !is_founded {
            token_type = match str.as_str() {
                "+" => Plus,
                "-" => Minus,
                "*" => Star,
                "/" => Slash,
                "=" => Eq,
                "!=" => NoEq,
                "==" => EqEq,
                ">" => Gt,
                ">=" => GtEq,
                "<" => Lt,
                "<=" => LtEq,
                "(" => LeftParen,
                ")" => RightParen,
                "{" => LeftBrace,
                "}" => RightBrace,
                "&&" => AndAnd,
                "||" => OrOr,
                "," => Comma,
                ":" => Colon,
                _ => panic!("Unknown operator: {}", str),
            };
        }

        self.tokens.push(Token {
            token_type,
            lexeme: Lexeme::None,
            start,
            end: self.pos
        });
    }

    fn lex_word(&mut self) {
        let start: usize = self.pos;

        let mut str: String = self.input[self.pos..].to_string();

        while self.peek().unwrap_or_default().is_ascii_alphanumeric() || self.peek().unwrap_or_default() == '_' {
            let symbol = self.peek().unwrap();
            str += &symbol.to_string();
            self.pos += 1;
        }

        let word = &self.input[start..self.pos];

        if self.keywords.contains_key(&word) {
            self.tokens.push(Token {
                token_type: *self.keywords.get(&word).unwrap(),
                lexeme: Lexeme::None,
                start,
                end: self.pos
            });
            return;
        }

        self.tokens.push(Token {
            token_type: TokenType::Word,
            lexeme: Lexeme::WordLexeme(word.to_string()),
            start,
            end: self.pos
        })
    }

    fn peek(&mut self) -> Option<char> {
        if self.pos < self.input.len() - 1 {
            let symbol = match self.input.chars().nth(self.pos) {
                Some(c) => c,
                None => return None,
            };
            return Some(symbol)
        }
        None
    }
}