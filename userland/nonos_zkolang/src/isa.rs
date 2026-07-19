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

//! zkolang VM instruction set, v0.1.
//!
//! One flat opcode per VM step, chosen so each maps to a small, fixed set of
//! algebraic constraints in the step AIR. There are no data-dependent jumps:
//! control is selection (`Sel`), not branching, so the trace shape is a function
//! of the program and not of the witness. Register operands are indices in
//! `0..REGS`.

use nonos_stark::field::Fp;

// Register file size. Fixed so the AIR's register columns are a constant width.
pub const REGS: usize = 16;

// A single VM instruction. `d` is the destination register; `a`, `b`, `c` are
// source registers.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    // r_d = literal.
    Imm { d: u8, v: Fp },
    // Field arithmetic.
    Add { d: u8, a: u8, b: u8 },
    Sub { d: u8, a: u8, b: u8 },
    Mul { d: u8, a: u8, b: u8 },
    // r_d = r_a^{-1}; an inverse of zero makes the trace unprovable.
    Inv { d: u8, a: u8 },
    // r_d = r_c ? r_a : r_b, with r_c constrained boolean. Branchless.
    Sel { d: u8, c: u8, a: u8, b: u8 },
    // r_d = (r_a == r_b) as {0,1}.
    Eq { d: u8, a: u8, b: u8 },
    // Constrain r_a to be boolean, or to be zero (assertion). No effect on
    // registers; a violated constraint yields no proof.
    Bool { a: u8 },
    Assert { a: u8 },
    // I/O against the public input vector and the public output vector.
    Inp { d: u8, idx: u16 },
    Out { a: u8, idx: u16 },
    // End of program.
    Halt,
}

// A compiled program: a flat instruction list. Bounded `for` loops are unrolled
// by the front-end before they reach here, so the list length is the step count.
pub type Program = [Op];
