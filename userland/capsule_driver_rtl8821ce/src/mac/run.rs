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

//! Execute a MAC init table against the register surface. Each operation is a
//! plain write, a set (read-or-write) or a clear (read-and-write) at its width.
//! This is pure control flow over the `Mmio` trait, so a host proof drives it
//! with a modeled device and checks the exact register program a table produces,
//! with no hardware.

use super::op::{Kind, MacOp, Width};
use crate::regs::Mmio;

/// Apply every operation in `table`, in order.
pub fn run_mac_table<M: Mmio>(mmio: &M, table: &[MacOp]) {
    for op in table {
        let off = op.offset as usize;
        match op.width {
            Width::B8 => {
                let v = op.value as u8;
                let next = match op.kind {
                    Kind::Write => v,
                    Kind::Set => mmio.read8(off) | v,
                    Kind::Clear => mmio.read8(off) & !v,
                };
                mmio.write8(off, next);
            }
            Width::B16 => {
                let v = op.value as u16;
                let next = match op.kind {
                    Kind::Write => v,
                    Kind::Set => mmio.read16(off) | v,
                    Kind::Clear => mmio.read16(off) & !v,
                };
                mmio.write16(off, next);
            }
            Width::B32 => {
                let v = op.value;
                let next = match op.kind {
                    Kind::Write => v,
                    Kind::Set => mmio.read32(off) | v,
                    Kind::Clear => mmio.read32(off) & !v,
                };
                mmio.write32(off, next);
            }
        }
    }
}
