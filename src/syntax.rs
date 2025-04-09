use crate::token::{LiteralType, Token};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        literal: LiteralType,
    },
    Unary {
        operator: Token,
        expression: Box<Expr>,
    },
}

pub trait Visitor {
    fn visit_binary();
    fn visit_grouping();
    fn visit_literal();
    fn visit_unary();
}
