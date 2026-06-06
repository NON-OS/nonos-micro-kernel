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

use super::info_type::ControllerInfo;
use crate::constants::{
    REG_AQA, REG_CAP, REG_CC, REG_CMBLOC, REG_CMBSZ, REG_CSTS, REG_INTMC, REG_INTMS, REG_VS,
};
use crate::regs::Regs;

impl ControllerInfo {
    pub fn read(regs: Regs) -> Self {
        Self {
            cap: unsafe { regs.r64(REG_CAP) },
            version: unsafe { regs.r32(REG_VS) },
            cc: unsafe { regs.r32(REG_CC) },
            csts: unsafe { regs.r32(REG_CSTS) },
            aqa: unsafe { regs.r32(REG_AQA) },
            intms: unsafe { regs.r32(REG_INTMS) },
            intmc: unsafe { regs.r32(REG_INTMC) },
            cmbloc: unsafe { regs.r32(REG_CMBLOC) },
            cmbsz: unsafe { regs.r32(REG_CMBSZ) },
        }
    }
}
