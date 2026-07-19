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

//! The per-row wiring: which register a row writes and which registers its three
//! read ports name. This is the public data flow of the program, and it becomes
//! the periodic one-hot columns the register binding reads. It is derived purely
//! from the opcode, because register indices are compile-time constants.

use crate::isa::Op;

/// The data flow of one row. A `None` port reads nothing and pins its operand to
/// zero; a `None` write leaves every register unchanged that step.
#[derive(Clone, Copy)]
pub(super) struct WireRow {
    pub(super) write: Option<u8>,
    pub(super) read_a: Option<u8>,
    pub(super) read_b: Option<u8>,
    pub(super) read_c: Option<u8>,
}

impl WireRow {
    /// A row that wires nothing, used for halt and padding rows.
    pub(super) const EMPTY: WireRow =
        WireRow { write: None, read_a: None, read_b: None, read_c: None };

    /// The wiring an opcode induces. Out-of-scope and halt rows wire nothing.
    pub(super) fn of(op: &Op) -> WireRow {
        match *op {
            Op::Imm { d, .. } => WireRow { write: Some(d), ..WireRow::EMPTY },
            Op::Inp { d, .. } => WireRow { write: Some(d), ..WireRow::EMPTY },
            Op::Add { d, a, b } | Op::Sub { d, a, b } | Op::Mul { d, a, b } => {
                WireRow { write: Some(d), read_a: Some(a), read_b: Some(b), read_c: None }
            }
            Op::Inv { d, a } => WireRow { write: Some(d), read_a: Some(a), ..WireRow::EMPTY },
            Op::Eq { d, a, b } => {
                WireRow { write: Some(d), read_a: Some(a), read_b: Some(b), read_c: None }
            }
            Op::Sel { d, c, a, b } => {
                WireRow { write: Some(d), read_a: Some(a), read_b: Some(b), read_c: Some(c) }
            }
            Op::Bool { a } | Op::Assert { a } => WireRow { read_a: Some(a), ..WireRow::EMPTY },
            Op::Out { a, .. } => WireRow { read_a: Some(a), ..WireRow::EMPTY },
            _ => WireRow::EMPTY,
        }
    }
}
