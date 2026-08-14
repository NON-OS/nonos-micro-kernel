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

use super::regs::{
    MAX_PLAUSIBLE_BUFFER, TPM_CRB_CTRL_CMD_HADDR, TPM_CRB_CTRL_CMD_LADDR, TPM_CRB_CTRL_CMD_SIZE,
    TPM_CRB_CTRL_RSP_HADDR, TPM_CRB_CTRL_RSP_LADDR, TPM_CRB_CTRL_RSP_SIZE,
};
use super::window::read32;
use crate::memory::addr::PhysAddr;
use crate::memory::unified::phys_to_virt;
use crate::security::tpm::error::TpmError;

/// Where the part expects a command, as the kernel can reach it.
///
/// The part publishes a physical address. The bootloader could write there
/// directly because it is identity mapped; the kernel resolves it through the
/// directmap first, and refuses if it does not land there rather than writing
/// to a physical address as though it were virtual.
pub(super) struct CrbBuffer {
    pub virt: u64,
    pub size: usize,
}

fn read_buffer(size_reg: u32, low_reg: u32, high_reg: u32) -> Result<CrbBuffer, TpmError> {
    let size = read32(size_reg)?;
    if size == 0 || size > MAX_PLAUSIBLE_BUFFER {
        return Err(TpmError::NotPresent);
    }
    let low = read32(low_reg)? as u64;
    let high = read32(high_reg)? as u64;
    let phys = (high << 32) | low;
    if phys == 0 {
        return Err(TpmError::NotPresent);
    }
    let virt = phys_to_virt(PhysAddr::new(phys)).ok_or(TpmError::NotPresent)?;
    Ok(CrbBuffer { virt: virt.as_u64(), size: size as usize })
}

pub(super) fn command_buffer() -> Result<CrbBuffer, TpmError> {
    read_buffer(TPM_CRB_CTRL_CMD_SIZE, TPM_CRB_CTRL_CMD_LADDR, TPM_CRB_CTRL_CMD_HADDR)
}

/// Often the same memory as the command buffer, which the spec allows. That is
/// why the command is written in full before the doorbell rings.
pub(super) fn response_buffer() -> Result<CrbBuffer, TpmError> {
    read_buffer(TPM_CRB_CTRL_RSP_SIZE, TPM_CRB_CTRL_RSP_LADDR, TPM_CRB_CTRL_RSP_HADDR)
}
