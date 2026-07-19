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

//! The compiler state and its register allocator. The allocator reuses freed
//! registers, and a name binding resolves newest-first with an alias-aware reclaim
//! on shadowing, which is what keeps register pressure at expression depth.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::parse::FnDef;
use super::super::CompileError;
use crate::isa::{Op, REGS};

// The largest number of iterations a single loop may unroll to. It guards against
// a stray large range building an enormous program; the trace-length cap catches
// anything that slips through at prove time, but this fails fast at compile.
pub(super) const MAX_UNROLL: u64 = 65_536;

// The deepest a chain of function calls may inline. A recursive call would inline
// without end, so this bound turns recursion into a compile error rather than a
// hang. Ordinary nesting is far below it.
pub(super) const MAX_INLINE: usize = 256;

pub(super) struct Compiler {
    pub(super) ops: Vec<Op>,
    // The defined functions, inlined at each call site.
    pub(super) fns: Vec<FnDef>,
    // The current depth of inlined calls, bounded by `MAX_INLINE`.
    pub(super) inline_depth: usize,
    pub(super) syms: Vec<(String, u8)>,
    // Active loop variables, innermost last. A loop variable is a compile-time
    // constant, so a reference to one lowers to an immediate rather than a
    // register read.
    pub(super) loop_consts: Vec<(String, u64)>,
    // The high-water mark of registers ever allocated, and the pool of registers
    // freed from dead temporaries and available for reuse.
    pub(super) next: u8,
    pub(super) free: Vec<u8>,
    // The number of public inputs; public inputs take indices `0..n_public` and
    // private (secret) inputs take indices from `n_public` on, so the public
    // inputs are a prefix the AIR binds and the secrets are a hidden suffix.
    pub(super) n_public: u16,
    pub(super) next_public: u16,
    pub(super) next_secret: u16,
    pub(super) next_output: u16,
}

/// A compiled subexpression: the register holding its value, and whether that
/// register is a temporary (safe to free once consumed) rather than a live binding.
pub(super) struct Val {
    pub(super) reg: u8,
    pub(super) temp: bool,
}

impl Compiler {
    /// A fresh compiler over a program's functions, with the public-input count
    /// already known so secret inputs can be indexed after the public prefix.
    pub(super) fn new(fns: Vec<FnDef>, n_public: u16) -> Compiler {
        Compiler {
            ops: Vec::new(),
            fns,
            inline_depth: 0,
            syms: Vec::new(),
            loop_consts: Vec::new(),
            next: 0,
            free: Vec::new(),
            n_public,
            next_public: 0,
            next_secret: 0,
            next_output: 0,
        }
    }

    /// Finish the program with a halt and hand back the instruction list.
    pub(super) fn finish(mut self) -> Vec<Op> {
        self.ops.push(Op::Halt);
        self.ops
    }

    /// Reserve a register, reusing a freed one when the pool is non-empty.
    pub(super) fn alloc(&mut self) -> Result<u8, CompileError> {
        if let Some(r) = self.free.pop() {
            return Ok(r);
        }
        if self.next as usize >= REGS {
            return Err(CompileError::TooManyRegisters);
        }
        let r = self.next;
        self.next += 1;
        Ok(r)
    }

    /// Return a value's register to the pool if it was a temporary.
    pub(super) fn release(&mut self, v: &Val) {
        if v.temp {
            self.free.push(v.reg);
        }
    }

    /// The register a bound name currently resolves to, newest binding first.
    pub(super) fn lookup(&self, name: &str) -> Option<u8> {
        self.syms.iter().rev().find(|(n, _)| n.as_str() == name).map(|(_, r)| *r)
    }

    /// The value of a loop variable if `name` is one, innermost loop first. A loop
    /// variable shadows a same-named binding while its loop is active.
    pub(super) fn loop_const(&self, name: &str) -> Option<u64> {
        self.loop_consts.iter().rev().find(|(n, _)| n.as_str() == name).map(|(_, v)| *v)
    }

    /// Point a name at a register, replacing its newest binding in place so no
    /// shadowed entry lingers to confuse the alias check, or adding it if new.
    pub(super) fn rebind(&mut self, name: &str, reg: u8) {
        if let Some(entry) = self.syms.iter_mut().rev().find(|(n, _)| n.as_str() == name) {
            entry.1 = reg;
        } else {
            self.syms.push((String::from(name), reg));
        }
    }

    /// Whether any live binding still holds this register, so it must not be freed.
    pub(super) fn reg_in_use(&self, reg: u8) -> bool {
        self.syms.iter().any(|(_, r)| *r == reg)
    }
}
