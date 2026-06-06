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
use crate::constants::{IC_DATA_CMD, IC_STATUS, IC_STATUS_RFNE};
use crate::regs::Regs;
use crate::transaction::TransferResult;

pub fn drain_rx(regs: Regs, out: &mut TransferResult, ri: &mut usize, read_len: usize) {
    while *ri < read_len && regs.read32(IC_STATUS) & IC_STATUS_RFNE != 0 {
        out.read[*ri] = (regs.read32(IC_DATA_CMD) & 0xFF) as u8;
        *ri += 1;
    }
}
