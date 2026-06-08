use super::ast::{AssignOp, Stmt, StmtKind};
use super::expr::{ArithOp, BExpr, CompOp, Expr, LogicOp, UnaryOp};
use crate::info;
use crate::lexer::token::{TKind, Token};
use std::mem::discriminant;

pub struct Parser<'a> {
    pos: usize,
    tokens: Vec<Token>,
    lines: Vec<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source: &'a str) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        Self {
            tokens,
            pos: 0,
            lines,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while self.peek(0).kind != TKind::Eof {
            let expr = self.stmt();
            stmts.push(expr);
        }
        stmts
    }

    pub fn parse_exprs(&mut self) -> Vec<Expr> {
        let mut exprs = vec![];
        while self.peek(0).kind != TKind::Eof {
            let expr = self.expr();
            exprs.push(expr);
        }
        exprs
    }

    pub fn error(&self, msg: &str, token: Token) -> String {
        let line = self.lines[token.line];
        let header = format!("Error in {}:{} - {msg}", token.line, token.offset);
        let err_line = format!("{} | {line}", token.line);
        let point = format!(
            "{} | {}{}",
            " ".repeat(token.line.to_string().len()),
            " ".repeat(token.offset),
            "^".repeat(token.len),
        );
        format!("{header}\n{err_line}\n{point}\n")
    }

    pub fn peek(&self, offset: i8) -> Token {
        let idx = self.pos + offset as usize;
        self.tokens.get(idx).unwrap().clone()
    }

    fn advance(&mut self, offset: u8) {
        self.pos += offset as usize;
    }

    fn check(&mut self, kind: TKind) -> bool {
        if discriminant(&self.peek(0).kind) == discriminant(&kind) {
            self.advance(1);
            true
        } else {
            false
        }
    }

    fn parse_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        while self.peek(0).kind != TKind::Eof {
            let value = self.expr();
            args.push(value);
            let current = self.peek(0);
            if current.kind == TKind::RParen {
                break;
            }
            if !self.check(TKind::Comma) {
                panic!(
                    "{}",
                    self.error(
                        &format!("Expected ')' or ',', found {:?}", current),
                        current
                    )
                );
            }
        }
        args
    }
}

impl Parser<'_> {
    fn expr(&mut self) -> Expr {
        self.logical()
    }
    fn logical(&mut self) -> Expr {
        let mut left = self.comparison();
        loop {
            let op_token = self.peek(0);
            let op = match op_token.kind {
                TKind::And => LogicOp::And,
                TKind::Or => LogicOp::Or,
                _ => break,
            };
            self.advance(1);
            let right = self.comparison();
            left = Expr::Logic(Box::new(left), op, Box::new(right), info!(op_token))
        }
        left
    }
    fn comparison(&mut self) -> Expr {
        let mut left = self.additive();
        loop {
            let op_token = self.peek(0);
            let op = match op_token.kind {
                TKind::Gt => CompOp::Gt,
                TKind::Ge => CompOp::Ge,
                TKind::Lt => CompOp::Lt,
                TKind::Le => CompOp::Le,
                TKind::Eq => CompOp::Eq,
                TKind::Ne => CompOp::Ne,
                _ => break,
            };
            self.advance(1);
            let right = self.additive();
            left = Expr::Comp(Box::new(left), op, Box::new(right), info!(op_token))
        }
        left
    }
    fn additive(&mut self) -> Expr {
        let mut left = self.multiplicative();
        loop {
            let op_token = self.peek(0);
            let op = match op_token.kind {
                TKind::Plus => ArithOp::Add,
                TKind::Minus => ArithOp::Sub,
                _ => break,
            };
            self.advance(1);
            let right = self.multiplicative();
            left = Expr::Arith(Box::new(left), op, Box::new(right), info!(op_token))
        }
        left
    }
    fn multiplicative(&mut self) -> Expr {
        let mut left = self.unary();
        loop {
            let op_token = self.peek(0);
            let op = match op_token.kind {
                TKind::Star => ArithOp::Mul,
                TKind::Slash => ArithOp::Div,
                _ => break,
            };
            self.advance(1);
            let right = self.unary();
            left = Expr::Arith(Box::new(left), op, Box::new(right), info!(op_token))
        }
        left
    }
    fn unary(&mut self) -> Expr {
        let current = self.peek(0);
        match current.kind {
            TKind::Minus => {
                self.advance(1);
                let primary = self.primary();
                let info = primary.info();
                let info = info!(info.line, info.offset - 1, info.len + 1);
                Expr::Unary(UnaryOp::Neg, Box::new(primary), info)
            }
            TKind::Bang => {
                self.advance(1);
                let primary = self.primary();
                let info = primary.info();
                let info = info!(info.line, info.offset - 1, info.len + 1);
                Expr::Unary(UnaryOp::Not, Box::new(primary), info)
            }
            _ => self.primary(),
        }
    }
    fn primary(&mut self) -> Expr {
        let current = self.peek(0);
        match current.kind {
            TKind::NumLit(n) => {
                self.advance(1);
                Expr::Num(n, info!(current))
            }
            TKind::StrLit(s) => {
                self.advance(1);
                Expr::Str(s, info!(current))
            }
            TKind::Id(id) => {
                self.advance(1);
                Expr::Id(id, info!(current))
            }
            TKind::Bool(truth) => {
                self.advance(1);
                Expr::Bool(truth, info!(current))
            }
            TKind::LParen => {
                self.advance(1);
                let expr = self.expr();
                if !self.check(TKind::RParen) {
                    panic!(
                        "{}",
                        self.error(&format!("Expected ')', found {:?}", current), current)
                    );
                }
                expr
            }
            _ => panic!("{}", self.error("Unexpected token in primary", current)),
        }
    }
}

impl Parser<'_> {
    fn stmt(&mut self) -> Stmt {
        match Stmt::define(self) {
            StmtKind::Assign => self.parse_assign(),
            StmtKind::Print => self.parse_print(),
        }
    }

    fn parse_assign(&mut self) -> Stmt {
        let id = match self.peek(0).kind {
            TKind::Id(id) => {
                self.advance(1);
                id
            }
            _ => unreachable!("{:?}", self.peek(0).kind),
        };
        let current = self.peek(0);
        let assign = match current.kind {
            TKind::Assign => AssignOp::default(),
            _ => panic!(
                "{}",
                self.error(&format!("Expected '=', found {:?}", current), current)
            ),
        };
        self.advance(1);
        let value = self.expr();
        Stmt::Assign(id, assign, value)
    }

    fn parse_print(&mut self) -> Stmt {
        self.advance(1);
        if !self.check(TKind::LParen) {
            let current = self.peek(0);
            panic!(
                "{}",
                self.error(&format!("Expected '(', found {:?}", current), current)
            );
        }
        let args = self.parse_args();
        if !self.check(TKind::RParen) {
            let current = self.peek(0);
            panic!(
                "{}",
                self.error(&format!("Expected ')', found {:?}", current), current)
            );
        }
        Stmt::Print(args)
    }
}
