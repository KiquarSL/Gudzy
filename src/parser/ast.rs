use super::Parser;
use super::expr::Expr;
use crate::lexer::token::TKind;

#[derive(Debug)]
pub enum Stmt {
    Assign(String, AssignOp, Expr),
    Print(Vec<Expr>),
}

#[derive(Debug)]
pub enum StmtKind {
    Assign,
    Print,
}

#[derive(Debug, Default)]
pub enum AssignOp {
    #[default]
    Assign,
}

impl Stmt {
    pub fn define(pr: &Parser) -> StmtKind {
        match (pr.peek(0).kind, pr.peek(1).kind) {
            (TKind::Id(_), TKind::Assign) => StmtKind::Assign,
            (TKind::Print, _) => StmtKind::Print,
            _ => panic!("{}", pr.error("Unknown statement", pr.peek(0))),
        }
    }
}
