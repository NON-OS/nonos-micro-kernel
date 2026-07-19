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

//! Lowering: the AST to a flat VM program. The language is single-assignment at
//! the source level, but the compiler reuses physical registers: once a temporary
//! subexpression value has been consumed by its parent, its register returns to a
//! free pool for the next temporary. This keeps register pressure at the depth of
//! the expression rather than its size, so a real program like a range proof fits
//! the sixteen-register file. Register indices stay compile-time constants, which
//! is all the step AIR's register binding needs, so reuse is invisible to it.
//!
//! The lowering is split by concern: the compiler state and register allocator,
//! the statement lowering, and the expression lowering (arithmetic, the witnessed
//! gadgets, the conditional, and inlined function calls).

mod compiler;
mod expr;
mod stmt;

use alloc::vec::Vec;

use compiler::Compiler;

use super::parse::{Ast, Stmt};
use super::CompileError;
use crate::isa::Op;

// The number of public inputs a statement list produces, counting the inputs a
// loop unrolls to, so the count matches the compiled program even through loops.
// In practice this stays small, because each input binds a register and the file
// holds only `REGS` of them.
fn count_inputs(stmts: &[Stmt]) -> u64 {
    let mut n = 0u64;
    for s in stmts {
        match s {
            Stmt::Input(_) => n += 1,
            Stmt::For { lo, hi, body, .. } => {
                let iters = hi.saturating_sub(*lo);
                n = n.saturating_add(iters.saturating_mul(count_inputs(body)));
            }
            _ => {}
        }
    }
    n
}

/// Lower an AST into a VM program ending in `Halt`.
pub fn compile(ast: &Ast) -> Result<Vec<Op>, CompileError> {
    // Count the public inputs first, through any loops, so secret inputs can be
    // indexed after them.
    let n_public = count_inputs(&ast.stmts).min(u16::MAX as u64) as u16;
    let mut c = Compiler::new(ast.fns.clone(), n_public);
    for s in &ast.stmts {
        c.stmt(s)?;
    }
    Ok(c.finish())
}
