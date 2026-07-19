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

//! The canonical byte encoding of a program: a version byte followed by one
//! fixed-width record per opcode, in order. It is versioned and fixed-width so the
//! encoding is reproducible across compilers and stable across time; a change to
//! any instruction changes the bytes, and so the digest over them.

use alloc::vec::Vec;

use crate::isa::Op;

// The encoding version. Bump it if the opcode encoding ever changes, so digests
// from different encodings never collide.
const VERSION: u8 = 1;

/// Canonically serialize a program to bytes: a version byte followed by one
/// fixed-width record per opcode, in order.
pub fn serialize(program: &[Op]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    out.push(VERSION);
    for op in program {
        match *op {
            Op::Imm { d, v } => {
                out.push(0x00);
                out.push(d);
                out.extend_from_slice(&v.value().to_le_bytes());
            }
            Op::Add { d, a, b } => out.extend_from_slice(&[0x01, d, a, b]),
            Op::Sub { d, a, b } => out.extend_from_slice(&[0x02, d, a, b]),
            Op::Mul { d, a, b } => out.extend_from_slice(&[0x03, d, a, b]),
            Op::Inv { d, a } => out.extend_from_slice(&[0x04, d, a]),
            Op::Sel { d, c, a, b } => out.extend_from_slice(&[0x05, d, c, a, b]),
            Op::Eq { d, a, b } => out.extend_from_slice(&[0x06, d, a, b]),
            Op::Bool { a } => out.extend_from_slice(&[0x07, a]),
            Op::Assert { a } => out.extend_from_slice(&[0x08, a]),
            Op::Inp { d, idx } => {
                out.push(0x09);
                out.push(d);
                out.extend_from_slice(&idx.to_le_bytes());
            }
            Op::Out { a, idx } => {
                out.push(0x0a);
                out.push(a);
                out.extend_from_slice(&idx.to_le_bytes());
            }
            Op::Halt => out.push(0x0b),
        }
    }
    out
}
