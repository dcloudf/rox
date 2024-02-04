#[derive(Debug, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String(String),
    Number(f32),

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Location,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

pub fn scan_tokens(input: String) -> Vec<Token> {
    let mut char_indices = input.char_indices().peekable();
    let mut output = Vec::new();
    let mut line: usize = 0;
    while let Some((position, ch)) = char_indices.next() {
        let token_type = match ch {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '!' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_) => Some(TokenType::BangEqual),
                None => Some(TokenType::Bang),
            },
            '=' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_) => Some(TokenType::EqualEqual),
                None => Some(TokenType::Equal),
            },
            '<' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_) => Some(TokenType::LessEqual),
                None => Some(TokenType::Less),
            },
            '>' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_) => Some(TokenType::GreaterEqual),
                None => Some(TokenType::Greater),
            },
            '/' => match char_indices.next_if_eq(&(position + 1, '/')) {
                Some(_) => {
                    char_indices.take_while(|(_, c)| *c != '\n');
                    None
                }
                None => Some(TokenType::Slash),
            },
            ' ' | '\r' | '\t' => None,
            '\n' => {
                line += 1;
                None
            }
            '"' => {
                let mut last_matched: char = '\0';
                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_, c)| {
                        last_matched = *c;
                        *c != '"'
                    })
                    .map(|(_, c)| c)
                    .collect();
                match last_matched {
                    '"' => Some(TokenType::String(s)),
                    '\n' => {
                        line += 1;
                        Some(TokenType::String(s))
                    }
                    _ => panic!(),
                }
            }
            ch if ch.is_digit(10) => {
                Some(
                    TokenType::Number(
                        char_indices.by_ref().take_while(|(_, c)| (*c).is_digit(10)).map(|(_, c)| c).collect::<String>().parse::<f32>().unwrap()
                    )
                )
            },
            ch if ch.is_alphabetic() => Some(TokenType::Identifier),
            _ => panic!(),
        };
    }
    output
}
