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
use crate::constants::{IC_CLR_TX_ABRT, IC_INTR_TX_ABRT, IC_RAW_INTR_STAT, IC_TX_ABRT_SOURCE};
use crate::regs::Regs;
use crate::transaction::{TransferError, TransferResult};

pub fn check_abort(regs: Regs, out: &mut TransferResult) -> Result<(), TransferError> {
    if regs.read32(IC_RAW_INTR_STAT) & IC_INTR_TX_ABRT == 0 {
        return Ok(());
    }
    out.abort_source = regs.read32(IC_TX_ABRT_SOURCE);
    let _ = regs.read32(IC_CLR_TX_ABRT);
    Err(TransferError::Nack)
}
