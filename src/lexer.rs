use std::str::Chars;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Num,
    Plus,
    Minus,
    Star,
    Slash,
    Eol,
    Let,
    Equal,
    Identifier,
    Lparen,
    Rparen,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub span: (usize, usize),
}
impl Token {
    pub fn new(kind: TokenType, span: (usize, usize)) -> Self {
        Self { kind, span }
    }
}
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
    source: String,
    size: usize,
}
impl<'a> Lexer<'a> {
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
    fn peek_advance(&mut self) -> Option<char> {
        self.advance();
        self.peek()
    }
    fn peek_next(&mut self) -> Option<char> {
        let mut cur_chars = self.chars.clone();
        cur_chars.next();
        cur_chars.next()
    }
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }
    fn index(&self) -> usize {
        self.size - self.chars.clone().count()
    }

    fn num(&mut self) -> Token {
        let mut dot_count: u16 = 0;
        let start = self.index();
        let mut current = self.peek();
        while let Some(val) = current {
            if !val.is_numeric() && val != '.' && val != '_' {
                break;
            }

            if val.eq(&'.') {
                let Some(next) = self.peek_next() else {
                    panic!("Invalid Number");
                };
                if !next.is_ascii_digit() {
                    break;
                }
                dot_count += 1;
            }
            current = self.peek_advance();
        }
        if dot_count > 1 {
            panic!("Invalid Number");
        }
        let is_float = dot_count != 0;
        if is_float {
            return Token::new(TokenType::Num, (start - 1, self.index()));
        }
        Token::new(TokenType::Num, (start - 1, self.index()))
    }
    pub fn next_tok(&mut self) -> Option<Token> {
        let start = self.index();
        let last = self.advance()?;
        let range = (start, start + 1);
        match last {
            '+' => Some(Token::new(TokenType::Plus, range)),
            '-' => Some(Token::new(TokenType::Minus, range)),
            ';' => Some(Token::new(TokenType::Eol, range)),
            '*' => Some(Token::new(TokenType::Star, range)),
            '/' => Some(Token::new(TokenType::Slash, range)),
            '=' => Some(Token::new(TokenType::Equal, range)),
            '(' => Some(Token::new(TokenType::Lparen, range)),
            ')' => Some(Token::new(TokenType::Rparen, range)),
            ' ' | '\t' | '\r' | '\n' => self.next_tok(),
            got => {
                if got.is_ascii_digit() {
                    return Some(self.num());
                }
                if got.is_ascii_alphabetic() {
                    return Some(self.ident());
                }
                return None;
            }
        }
    }
    fn ident(&mut self) -> Token {
        let start = self.index() - 1;
        let mut current = self.peek();
        while let Some(val) = current {
            if !val.is_alphanumeric() && val != '_' {
                break;
            }
            current = self.peek_advance();
        }
        let stop = self.index();
        let span = &self.source[start..stop];
        let Some(keyword) = self.map_keyword(span.to_string()) else {
            return Token::new(TokenType::Identifier, (start, stop));
        };
        Token::new(keyword, (start, stop))
    }
    fn map_keyword(&self, text: String) -> Option<TokenType> {
        match text.as_str() {
            "let" => Some(TokenType::Let),
            _ => None,
        }
    }
    pub fn new(src: &'a str) -> Self {
        return Self {
            chars: src.chars(),
            source: String::from(src),
            size: src.chars().count(),
        };
    }
}
