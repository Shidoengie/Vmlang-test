use crate::astnodes::*;
use crate::lexer;
use lexer::*;
use std::iter::Peekable;
#[derive(Clone)]
pub struct TokenIter<'input> {
    lexer: Lexer<'input>,
}

impl<'input> TokenIter<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
}

impl<'input> Iterator for TokenIter<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_tok()
    }
}
#[derive(Clone)]
pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str) -> Parser<'input, TokenIter<'input>> {
        Parser {
            input,
            tokens: TokenIter::new(input).peekable(),
        }
    }
    pub fn text(&mut self, token: &Token) -> String {
        self.input[token.span.0..token.span.1].to_string()
    }
    fn peek(&mut self) -> Option<Token> {
        self.tokens.peek().cloned()
    }
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    fn consume(&mut self, expected: TokenType) -> Token {
        let consumed = self.next().expect("Expected a token");
        if consumed.kind == expected {
            return consumed;
        }
        panic!(
            "expected {expected:?} but got {:?} stupid ass",
            consumed.kind
        );
    }
    pub fn atom_parser(&mut self) -> Node {
        let maybe_val = self.peek().unwrap();
        self.next();
        match maybe_val.kind {
            TokenType::Lparen => {
                let val = self.parse();
                return val;
            }
            TokenType::Let => {
                let ident = self.consume(TokenType::Identifier);
                let name = self.text(&ident);
                dbg!(self.peek());
                self.consume(TokenType::Equal);
                let expr = self.parse();
                return Node::Def(name, Box::new(expr));
            }
            TokenType::Minus => Node::UnaryNode(UnaryOp::Negate, Box::new(self.atom_parser())),
            TokenType::Identifier => Node::Ident(self.text(&maybe_val)),
            TokenType::Num => Node::Value(self.text(&maybe_val).parse().unwrap()),
            dumb => panic!("bitch wtf you think you doing... btw invalid atom fucko {dumb:?}"),
        }
    }

    pub fn parse(&mut self) -> Node {
        let left = self.atom_parser();
        let Some(op) = self.peek() else{ return left;};
        self.next();
        match op.kind {
            TokenType::Num => panic!("wth are you doing invalid operator btw bitch"),
            TokenType::Equal => {
                let Node::Ident(name) = left else{
                    panic!("invalid assignment target bitch")
                };
                let expr = self.parse();
                Node::Assign(name, Box::new(expr))
            }
            TokenType::Minus => {
                Node::BinaryNode(BinaryOp::Subtract, Box::new(left), Box::new(self.parse()))
            }
            TokenType::Plus => {
                Node::BinaryNode(BinaryOp::Add, Box::new(left), Box::new(self.parse()))
            }
            TokenType::Star => {
                Node::BinaryNode(BinaryOp::Multiply, Box::new(left), Box::new(self.parse()))
            }
            TokenType::Slash => {
                Node::BinaryNode(BinaryOp::Divide, Box::new(left), Box::new(self.parse()))
            }
            _ => return left,
        }
    }
}
