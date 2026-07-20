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

//! The rtw88 power-sequence command, the unit that every Realtek MAC power
//! transition (card-disable to card-enable, active, LPS) is expressed in. A
//! transition is a table of these commands the executor walks in order. The
//! command shape mirrors rtw88's `rtw_pwr_seq_cmd`; the tables themselves are
//! vendor data ported verbatim from rtw88 (GPL) with attribution, never
//! reconstructed here. The 8821CE PCIe path uses register writes, readiness
//! polls and the end marker; rtw88's fixed millisecond delays fall only on the
//! USB and SDIO interfaces, so the delay command is not part of this driver.

/// Modify a register: `reg = (reg & !mask) | (value & mask)`.
pub const CMD_WRITE: u8 = 0;
/// Poll until `reg & mask == value & mask`, up to a bounded number of reads.
pub const CMD_POLL: u8 = 1;
/// End of the sequence.
pub const CMD_END: u8 = 3;

/// One power-sequence step. `offset` is a byte-register offset in the MAC block.
#[derive(Clone, Copy)]
pub struct PwrCmd {
    pub offset: u16,
    pub cmd: u8,
    pub mask: u8,
    pub value: u8,
}

impl PwrCmd {
    pub const fn write(offset: u16, mask: u8, value: u8) -> Self {
        Self { offset, cmd: CMD_WRITE, mask, value }
    }
    pub const fn poll(offset: u16, mask: u8, value: u8) -> Self {
        Self { offset, cmd: CMD_POLL, mask, value }
    }
    pub const fn end() -> Self {
        Self { offset: 0, cmd: CMD_END, mask: 0, value: 0 }
    }
}
