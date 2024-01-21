use std::collections::HashMap;

use crate::token::{TokenType, Token};
use crate::error;


pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("fun".to_string(), TokenType::Fun);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("nil".to_string(), TokenType::Nil);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("print".to_string(), TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("this".to_string(), TokenType::This);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);

        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), "".to_string(), self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count() as i32
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token(match self.match_char('=') {true => TokenType::BangEqual, false => TokenType::Bang}),
            '=' => self.add_token(match self.match_char('=') {true => TokenType::EqualEqual, false => TokenType::Equal}),
            '<' => self.add_token(match self.match_char('=') {true => TokenType::LessEqual, false => TokenType::Less}),
            '>' => self.add_token(match self.match_char('=') {true => TokenType::GreaterEqual, false => TokenType::Greater}),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    error(self.line, "Unexpected character.".to_string())
                }
            },
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source.as_str()[self.start as usize..self.current as usize];
        let token_type = self.keywords.get(text);
        match token_type {
            Some(k) => self.add_token(k.clone()),
            None => self.add_token(TokenType::Identifier)
        }
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token_with_literal(
            TokenType::Number,
            &self.source.as_str()[self.start as usize..self.current as usize],
        )
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.chars().count() as i32 {
            return '\0'
        }
        self.source.chars().nth((self.current + 1) as usize).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated string.".to_string());
            return;
        }
        self.advance();
        let value = &self.source.as_str()[self.start as usize..self.current as usize];
        self.add_token_with_literal(TokenType::String, value)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        return self.source.chars().nth(self.current as usize).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() { return true; }
        if self.source.chars().nth(self.current as usize).unwrap() != expected { return false; }
        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, "")
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: &str) {
        let text = &self.source.as_str()[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(token_type, String::from(text), literal.to_string(), self.line))
    }
}
