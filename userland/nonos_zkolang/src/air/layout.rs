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

//! The trace column layout and the AIR's fixed sizes, in one place so every other
//! file names a column by the same constant. The trace is a matrix: a clock, the
//! one-hot opcode selectors, three read operands, a result, an immediate, an
//! auxiliary witness, and the register file threaded after them.

use crate::isa::REGS;

// Step columns. One clock counter, twelve one-hot opcode selectors, three read
// operands, one result, one immediate, one auxiliary witness.
pub(super) const CLK: usize = 0;
pub(super) const S_IMM: usize = 1;
pub(super) const S_ADD: usize = 2;
pub(super) const S_SUB: usize = 3;
pub(super) const S_MUL: usize = 4;
pub(super) const S_INV: usize = 5;
pub(super) const S_EQ: usize = 6;
pub(super) const S_SEL: usize = 7;
pub(super) const S_BOOL: usize = 8;
pub(super) const S_ASSERT: usize = 9;
pub(super) const S_INP: usize = 10;
pub(super) const S_OUT: usize = 11;
pub(super) const S_HALT: usize = 12;
pub(super) const A: usize = 13;
pub(super) const B: usize = 14;
pub(super) const C: usize = 15;
pub(super) const D: usize = 16;
pub(super) const IMM: usize = 17;
pub(super) const AUX: usize = 18;

// The register file occupies the columns after the step columns: `REGS` columns
// holding the register state before the row executes.
pub(super) const RF_BASE: usize = 19;

/// The width of the step trace: the step columns plus the register file.
pub const TRACE_WIDTH: usize = RF_BASE + REGS;

// Periodic (public) wiring columns, in this order: the write one-hot, then the
// three read-port one-hots, each `REGS` wide.
pub(super) const WRITE_P: usize = 0;
pub(super) const READA_P: usize = REGS;
pub(super) const READB_P: usize = 2 * REGS;
pub(super) const READC_P: usize = 3 * REGS;
pub(super) const NUM_PERIODIC: usize = 4 * REGS;

// The window is a row and its successor, so the ordering and write-propagation
// constraints can read the next row.
pub(super) const WINDOW: usize = 2;

// Transition constraint count: 27 step constraints, three read bindings, and one
// write propagation per register.
pub(super) const NUM_TRANSITION: usize = 27 + 3 + REGS;

// Highest degree among the constraints, e.g. the multiply gate or the witnessed
// equality `s_eq * (d + diff*aux - 1)`. The register bindings are linear.
pub(super) const DEGREE: usize = 3;
