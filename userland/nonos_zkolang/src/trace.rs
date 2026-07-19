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

//! The execution trace: one row per VM step, recording every value a transition
//! constraint references. The VM fills these rows as it runs and the AIR reads
//! them, so a run and its proof agree on the same object with nothing hidden.
//! The step-AIR column layout is derived from this row by the AIR module.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

// One VM step. The `op` tag selects which constraint set is active for the row.
#[derive(Clone, Copy, Debug)]
pub struct Row {
    // Step counter; row 0 is the boundary. Increments by one.
    pub clk: u64,
    // Opcode tag for the selector column.
    pub op: OpTag,
    // Register values read (a, b, c) and written (d) this step.
    pub ra: Fp,
    pub rb: Fp,
    pub rc: Fp,
    pub rd: Fp,
    // Immediate operand, when the op carries one.
    pub imm: Fp,
    // Auxiliary witness: the inverse for Inv and Eq, the tested value for Bool
    // and Assert. Zero when unused.
    pub aux: Fp,
}

impl Row {
    // A zeroed row at a given clock, filled in by the executor per opcode.
    pub fn at(clk: u64) -> Row {
        Row {
            clk,
            op: OpTag::Halt,
            ra: Fp::ZERO,
            rb: Fp::ZERO,
            rc: Fp::ZERO,
            rd: Fp::ZERO,
            imm: Fp::ZERO,
            aux: Fp::ZERO,
        }
    }
}

// The opcode selector, one tag per instruction. The AIR turns this into the
// one-hot selector columns that gate each opcode's transition constraints.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpTag {
    Imm,
    Add,
    Sub,
    Mul,
    Inv,
    Sel,
    Eq,
    Bool,
    Assert,
    Inp,
    Out,
    Halt,
}

// A full execution trace plus the public boundary the proof commits to.
pub struct Trace {
    pub rows: Vec<Row>,
    pub public_inputs: Vec<Fp>,
    pub public_outputs: Vec<Fp>,
}
