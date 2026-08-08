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

use crate::hardware::tpm::constants::{
    TPM_CRB_CTRL_CMD_HADDR, TPM_CRB_CTRL_CMD_LADDR, TPM_CRB_CTRL_CMD_SIZE, TPM_CRB_CTRL_RSP_HADDR,
    TPM_CRB_CTRL_RSP_LADDR, TPM_CRB_CTRL_RSP_SIZE,
};
use crate::hardware::tpm::state::TpmState;
use crate::hardware::tpm::types::TpmError;

/// Where a CRB part expects a command to be written, and how much will fit.
pub(crate) struct CrbBuffer {
    pub addr: u64,
    pub size: usize,
}

/// Largest buffer this driver will believe. The registers are part-supplied, so
/// a wrong or malicious value would otherwise become the length of a memcpy
/// into an arbitrary physical address. Real parts advertise a few kilobytes.
const MAX_PLAUSIBLE: u32 = 0x1_0000;

fn read_buffer(state: &TpmState, size_reg: u32, low_reg: u32, high_reg: u32) -> Option<CrbBuffer> {
    let size = state.read_reg32(size_reg);
    if size == 0 || size > MAX_PLAUSIBLE {
        return None;
    }

    let low = state.read_reg32(low_reg) as u64;
    let high = state.read_reg32(high_reg) as u64;
    let addr = (high << 32) | low;
    if addr == 0 {
        return None;
    }

    Some(CrbBuffer { addr, size: size as usize })
}

/// The command buffer the part published.
pub(crate) fn command_buffer(state: &TpmState) -> Result<CrbBuffer, TpmError> {
    read_buffer(state, TPM_CRB_CTRL_CMD_SIZE, TPM_CRB_CTRL_CMD_LADDR, TPM_CRB_CTRL_CMD_HADDR)
        .ok_or(TpmError::NotPresent)
}

/// The response buffer the part published. Often the same memory as the
/// command buffer, which is allowed and is why the command is fully written
/// before the doorbell rings.
pub(crate) fn response_buffer(state: &TpmState) -> Result<CrbBuffer, TpmError> {
    read_buffer(state, TPM_CRB_CTRL_RSP_SIZE, TPM_CRB_CTRL_RSP_LADDR, TPM_CRB_CTRL_RSP_HADDR)
        .ok_or(TpmError::NotPresent)
}
