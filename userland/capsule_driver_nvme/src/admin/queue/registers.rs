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

use super::constants::ADMIN_ENTRIES;
use super::types::AdminQueue;
use crate::constants::{REG_ACQ, REG_AQA, REG_ASQ};
use crate::regs::Regs;

impl AdminQueue {
    pub fn program_registers(&self, regs: Regs) {
        unsafe {
            regs.w32(REG_AQA, ((ADMIN_ENTRIES as u32 - 1) << 16) | (ADMIN_ENTRIES as u32 - 1));
            regs.w64(REG_ASQ, self.sq.device_addr());
            regs.w64(REG_ACQ, self.cq.device_addr());
        }
    }
}
