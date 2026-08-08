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

use super::super::types::{IrqBindError, IrqBindRequest, IrqBindResult, BIND_MSIX};
use super::intx::bind_intx;
use super::msix::bind_msix;
use crate::hardware::broker::claim;

pub fn bind(pid: u32, req: IrqBindRequest) -> Result<IrqBindResult, IrqBindError> {
    let claim = claim::lookup(req.device_id).ok_or(IrqBindError::NotClaimed)?;
    if claim.pid != pid {
        return Err(IrqBindError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(IrqBindError::StaleEpoch);
    }

    if req.flags & BIND_MSIX != 0 {
        bind_msix(pid, req, claim.epoch)
    } else {
        bind_intx(pid, req, claim.epoch)
    }
}
