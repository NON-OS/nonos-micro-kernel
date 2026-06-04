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

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_ENABLE_SLOT, STATUS_LEN};

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_LEN: usize = 4;

pub fn enable_slot(xhci_port: u32) -> Result<u8, XhciClientError> {
    let mut resp = [0u8; HDR_AND_STATUS + REPLY_LEN];
    let (status, data_len) = call(xhci_port, OP_ENABLE_SLOT, &[], &mut resp)?;
    if status != 0 || data_len < REPLY_LEN {
        return Err(XhciClientError::BadResponse);
    }
    let slot = resp[HDR_AND_STATUS];
    if slot == 0 {
        return Err(XhciClientError::BadResponse);
    }
    Ok(slot)
}
