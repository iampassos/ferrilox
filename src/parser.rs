use crate::{
    syntax::Expr,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression(&self) {
        return self.equality();
    }

    pub fn equality(&self) {
        let mut expr = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = *self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: expr,
                operator,
                right,
            };
        }

        expr
    }

    pub fn matches(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    pub fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ttype == *ttype
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    pub fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap_or(self.peek())
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
}
