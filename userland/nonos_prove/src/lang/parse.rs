// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Recursive-descent parser: a token stream to the abstract syntax tree the
//! compiler lowers. Precedence is encoded by the call chain, equality lowest,
//! then add and subtract, then multiply, then primaries.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::lex::Tok;
use super::CompileError;

// An expression node.
#[derive(Clone, Debug)]
pub enum Expr {
    Num(u64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Inv(Box<Expr>),
    Sel(Box<Expr>, Box<Expr>, Box<Expr>),
}

// A statement node.
#[derive(Clone, Debug)]
pub enum Stmt {
    Let(String, Expr),
    Assert(Expr),
    // Bind a name to the next public input.
    Input(String),
    // Expose an expression as the next public output.
    Output(Expr),
}

/// A parsed program.
#[derive(Clone, Debug)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}

struct Parser<'a> {
    toks: &'a [Tok],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn bump(&mut self) -> Option<&Tok> {
        let t = self.toks.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    // Consume a token that must be exactly `want`.
    fn expect(&mut self, want: &Tok) -> Result<(), CompileError> {
        match self.bump() {
            Some(t) if t == want => Ok(()),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    fn program(&mut self) -> Result<Ast, CompileError> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.stmt()?);
        }
        Ok(Ast { stmts })
    }

    fn stmt(&mut self) -> Result<Stmt, CompileError> {
        match self.peek() {
            Some(Tok::Let) => {
                self.pos += 1;
                let name = match self.bump() {
                    Some(Tok::Ident(n)) => n.clone(),
                    Some(_) => return Err(CompileError::UnexpectedToken),
                    None => return Err(CompileError::UnexpectedEof),
                };
                self.expect(&Tok::Assign)?;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Let(name, e))
            }
            Some(Tok::Assert) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Assert(e))
            }
            Some(Tok::Input) => {
                self.pos += 1;
                let name = match self.bump() {
                    Some(Tok::Ident(n)) => n.clone(),
                    Some(_) => return Err(CompileError::UnexpectedToken),
                    None => return Err(CompileError::UnexpectedEof),
                };
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Input(name))
            }
            Some(Tok::Output) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Output(e))
            }
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, CompileError> {
        let lhs = self.sum()?;
        if matches!(self.peek(), Some(Tok::EqEq)) {
            self.pos += 1;
            let rhs = self.sum()?;
            return Ok(Expr::Eq(Box::new(lhs), Box::new(rhs)));
        }
        Ok(lhs)
    }

    fn sum(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.product()?;
        loop {
            match self.peek() {
                Some(Tok::Plus) => {
                    self.pos += 1;
                    let rhs = self.product()?;
                    lhs = Expr::Add(Box::new(lhs), Box::new(rhs));
                }
                Some(Tok::Minus) => {
                    self.pos += 1;
                    let rhs = self.product()?;
                    lhs = Expr::Sub(Box::new(lhs), Box::new(rhs));
                }
                _ => return Ok(lhs),
            }
        }
    }

    fn product(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.primary()?;
        while matches!(self.peek(), Some(Tok::Star)) {
            self.pos += 1;
            let rhs = self.primary()?;
            lhs = Expr::Mul(Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn primary(&mut self) -> Result<Expr, CompileError> {
        match self.bump() {
            Some(Tok::Num(v)) => Ok(Expr::Num(*v)),
            Some(Tok::Ident(n)) => Ok(Expr::Var(n.clone())),
            Some(Tok::LParen) => {
                let e = self.expr()?;
                self.expect(&Tok::RParen)?;
                Ok(e)
            }
            Some(Tok::Inv) => {
                self.expect(&Tok::LParen)?;
                let e = self.expr()?;
                self.expect(&Tok::RParen)?;
                Ok(Expr::Inv(Box::new(e)))
            }
            Some(Tok::Sel) => {
                self.expect(&Tok::LParen)?;
                let cond = self.expr()?;
                self.expect(&Tok::Comma)?;
                let a = self.expr()?;
                self.expect(&Tok::Comma)?;
                let b = self.expr()?;
                self.expect(&Tok::RParen)?;
                Ok(Expr::Sel(Box::new(cond), Box::new(a), Box::new(b)))
            }
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }
}

/// Parse a token stream into an AST.
pub fn parse(toks: &[Tok]) -> Result<Ast, CompileError> {
    let mut p = Parser { toks, pos: 0 };
    p.program()
}
