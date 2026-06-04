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
use crate::constants::{IC_ENABLE, IC_ENABLE_ENABLE, IC_TAR};
use crate::regs::Regs;
use crate::transaction::TransferError;

use super::disable::disable;
use super::enable::enable;

pub fn set_target(regs: Regs, addr: u8) -> Result<(), TransferError> {
    let was_enabled = regs.read32(IC_ENABLE) & IC_ENABLE_ENABLE != 0;
    if was_enabled {
        disable(regs)?;
    }
    regs.write32(IC_TAR, (addr & 0x7F) as u32);
    if was_enabled {
        enable(regs)?;
    }
    Ok(())
}
