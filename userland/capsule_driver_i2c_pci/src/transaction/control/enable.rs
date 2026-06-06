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
use crate::constants::IC_ENABLE;
use crate::constants::IC_ENABLE_ENABLE;
use crate::regs::Regs;
use crate::transaction::TransferError;

use super::wait_enable_state::wait_enable_state;

pub fn enable(regs: Regs) -> Result<(), TransferError> {
    regs.write32(IC_ENABLE, IC_ENABLE_ENABLE);
    wait_enable_state(regs, true)
}
