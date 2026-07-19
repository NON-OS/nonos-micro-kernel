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

// An expression node. The tree is what the compiler walks; each variant lowers to
// a small, fixed run of opcodes, which is why the set stays this compact.
#[derive(Clone, Debug)]
pub enum Expr {
    // A literal and a variable reference, the leaves of every tree.
    Num(u64),
    Var(String),
    // Field arithmetic. `Div` is sugar for a multiply by an inverse, and `Neg` for
    // a subtraction from zero, so neither needs its own opcode.
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    // Comparisons that yield a zero or one bit. `Ne` is the complement of `Eq`,
    // lowered as one minus the equality bit.
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    // The field inverse and the branchless select, written as calls.
    Inv(Box<Expr>),
    Sel(Box<Expr>, Box<Expr>, Box<Expr>),
    // A conditional expression, sugar for `sel`: both arms are evaluated and one
    // is chosen by the boolean condition. Order is (cond, then, else).
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

// A statement node.
#[derive(Clone, Debug)]
pub enum Stmt {
    Let(String, Expr),
    Assert(Expr),
    // Bind a name to the next public input.
    Input(String),
    // Bind a name to the next private input, a witness not in the public statement.
    Secret(String),
    // Expose an expression as the next public output.
    Output(Expr),
    // A bounded loop over `[lo, hi)`, unrolled by the compiler. The loop variable
    // is a compile-time constant in the body, so the body's shape never depends on
    // a runtime value.
    For { var: String, lo: u64, hi: u64, body: Vec<Stmt> },
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
            Some(Tok::Secret) => {
                self.pos += 1;
                let name = match self.bump() {
                    Some(Tok::Ident(n)) => n.clone(),
                    Some(_) => return Err(CompileError::UnexpectedToken),
                    None => return Err(CompileError::UnexpectedEof),
                };
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Secret(name))
            }
            Some(Tok::Output) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Output(e))
            }
            Some(Tok::For) => self.for_loop(),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    // A bounded loop: `for i in lo .. hi { stmt* }`. The bounds are literals, so
    // the iteration count is known at compile time and the compiler unrolls it.
    fn for_loop(&mut self) -> Result<Stmt, CompileError> {
        self.pos += 1; // the `for`
        let var = match self.bump() {
            Some(Tok::Ident(n)) => n.clone(),
            Some(_) => return Err(CompileError::UnexpectedToken),
            None => return Err(CompileError::UnexpectedEof),
        };
        self.expect(&Tok::In)?;
        let lo = self.number()?;
        self.expect(&Tok::DotDot)?;
        let hi = self.number()?;
        self.expect(&Tok::LBrace)?;
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Tok::RBrace)) {
            if self.peek().is_none() {
                return Err(CompileError::UnexpectedEof);
            }
            body.push(self.stmt()?);
        }
        self.expect(&Tok::RBrace)?;
        Ok(Stmt::For { var, lo, hi, body })
    }

    // Consume a single numeric literal, for a loop bound.
    fn number(&mut self) -> Result<u64, CompileError> {
        match self.bump() {
            Some(Tok::Num(v)) => Ok(*v),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }

    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.equality()
    }

    // The lowest precedence: an optional single comparison of two sums. It is not
    // chainable, `a == b == c` is a parse error, because a comparison yields a bit
    // and comparing a bit to a third sum is almost never what a writer means.
    fn equality(&mut self) -> Result<Expr, CompileError> {
        let lhs = self.sum()?;
        match self.peek() {
            Some(Tok::EqEq) => {
                self.pos += 1;
                let rhs = self.sum()?;
                Ok(Expr::Eq(Box::new(lhs), Box::new(rhs)))
            }
            Some(Tok::BangEq) => {
                self.pos += 1;
                let rhs = self.sum()?;
                Ok(Expr::Ne(Box::new(lhs), Box::new(rhs)))
            }
            _ => Ok(lhs),
        }
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

    // Multiply and divide, which bind tighter than add and subtract. Both are
    // left-associative, so `a / b / c` is `(a / b) / c`. Their operands are
    // unary expressions, so a leading minus binds tighter still.
    fn product(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.unary()?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.pos += 1;
                    let rhs = self.unary()?;
                    lhs = Expr::Mul(Box::new(lhs), Box::new(rhs));
                }
                Some(Tok::Slash) => {
                    self.pos += 1;
                    let rhs = self.unary()?;
                    lhs = Expr::Div(Box::new(lhs), Box::new(rhs));
                }
                _ => return Ok(lhs),
            }
        }
    }

    // A prefix minus negates. It is right-recursive so `- - a` double-negates, and
    // it sits above the primaries so `-a * b` parses as `(-a) * b`.
    fn unary(&mut self) -> Result<Expr, CompileError> {
        if matches!(self.peek(), Some(Tok::Minus)) {
            self.pos += 1;
            let inner = self.unary()?;
            return Ok(Expr::Neg(Box::new(inner)));
        }
        self.primary()
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
            // A conditional expression: `if c { a } else { b }`. Both arms are
            // single expressions, because the lowering to `sel` evaluates both.
            Some(Tok::If) => {
                let cond = self.expr()?;
                self.expect(&Tok::LBrace)?;
                let a = self.expr()?;
                self.expect(&Tok::RBrace)?;
                self.expect(&Tok::Else)?;
                self.expect(&Tok::LBrace)?;
                let b = self.expr()?;
                self.expect(&Tok::RBrace)?;
                Ok(Expr::If(Box::new(cond), Box::new(a), Box::new(b)))
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
