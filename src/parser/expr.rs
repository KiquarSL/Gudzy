use super::Info;
use std::fmt;

pub type BExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64, Info),
    Str(String, Info),
    Id(String, Info),
    Bool(bool, Info),
    Unary(UnaryOp, BExpr, Info),
    Arith(BExpr, ArithOp, BExpr, Info),
    Comp(BExpr, CompOp, BExpr, Info),
    Logic(BExpr, LogicOp, BExpr, Info),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Num(n, _) => write!(f, "{n}"),
            Expr::Str(s, _) => write!(f, "\"{s}\""),
            Expr::Id(id, _) => write!(f, "{id}"),
            Expr::Bool(b, _) => write!(f, "{b}"),
            Expr::Arith(l, op, r, _) => write!(f, "({l} {op} {r})"),
            Expr::Comp(l, op, r, _) => write!(f, "({l} {op} {r})"),
            Expr::Logic(l, op, r, _) => write!(f, "({l} {op} {r})"),
            Expr::Unary(op, expr, _) => write!(f, "{op}{expr}"),
        }
    }
}

impl Expr {
    pub fn info(&self) -> Info {
        match self {
            Expr::Str(_, info) => info.clone(),
            Expr::Num(_, info) => info.clone(),
            Expr::Id(_, info) => info.clone(),
            Expr::Bool(_, info) => info.clone(),
            Expr::Arith(_, _, _, info) => info.clone(),
            Expr::Comp(_, _, _, info) => info.clone(),
            Expr::Logic(_, _, _, info) => info.clone(),
            Expr::Unary(_, _, info) => info.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ArithOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

#[derive(Debug, Clone)]
pub enum CompOp {
    Gt, // >
    Ge, // >=
    Lt, // <
    Le, // <=
    Eq, // ==
    Ne, // !=
}

#[derive(Debug, Clone)]
pub enum LogicOp {
    And, // &&
    Or,  // ||
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not, // !
    Neg, // -
}

impl fmt::Display for ArithOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithOp::Add => write!(f, "+"),
            ArithOp::Sub => write!(f, "-"),
            ArithOp::Mul => write!(f, "*"),
            ArithOp::Div => write!(f, "/"),
        }
    }
}

impl fmt::Display for CompOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompOp::Gt => write!(f, ">"),
            CompOp::Ge => write!(f, ">="),
            CompOp::Lt => write!(f, "<"),
            CompOp::Le => write!(f, "<="),
            CompOp::Eq => write!(f, "=="),
            CompOp::Ne => write!(f, "!="),
        }
    }
}

impl fmt::Display for LogicOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicOp::And => write!(f, "&&"),
            LogicOp::Or => write!(f, "||"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}
