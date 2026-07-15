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
use crate::constants::{PORTSC_BASE, PORTSC_CHANGE_BITS, PORTSC_PP, PORT_REG_STRIDE};
use crate::regs::mmio_write32;

// RW bits we must preserve across a change-bit acknowledge. PP (port power)
// is RW, not RW1C: a bare write of the change bits would clear it and power
// the port off. We must never re-assert PR (port reset) or PED (writing 1
// disables the port) or the PLS write strobe, so only PP is carried over.
const PORTSC_PRESERVE_RW: u32 = PORTSC_PP;

pub fn portsc_clear_changes(op_base: u64, port: u8, snapshot: u32) {
    let reg = op_base + PORTSC_BASE + ((port as u64) - 1) * PORT_REG_STRIDE;
    let to_clear = snapshot & PORTSC_CHANGE_BITS;
    if to_clear == 0 {
        return;
    }
    let preserved = snapshot & PORTSC_PRESERVE_RW;
    mmio_write32(reg, preserved | to_clear);
}
