use crate::token::{LiteralType, Token};

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

struct Grouping {
    expression: Box<Expr>,
}

struct Literal {
    literal: LiteralType,
}

struct Unary {
    operator: Token,
    expression: Box<Expr>,
}

trait Visitor {
    fn visit_binary();
    fn visit_grouping();
    fn visit_literal();
    fn visit_unary();
}
