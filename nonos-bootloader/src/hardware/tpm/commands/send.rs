// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::hardware::tpm::constants::{TPM_DATA_FIFO, TPM_STS, TPM_STS_GO, TPM_STS_READY};
use crate::hardware::tpm::state::TpmState;
use crate::hardware::tpm::types::TpmError;

pub fn send_command_impl(state: &TpmState, cmd: &[u8]) -> Result<(), TpmError> {
    if !state.initialized {
        return Err(TpmError::NotPresent);
    }
    // A CRB part takes commands through a buffer in memory, not a byte port.
    // Handing it the FIFO sequence writes command bytes across its control
    // registers, which is what made every reply from one look invalid.
    if state.is_crb {
        return super::crb_send(state, cmd);
    }
    state.write_reg8(TPM_STS, TPM_STS_READY);
    state.wait_for_status(TPM_STS_READY, TPM_STS_READY)?;
    for byte in cmd {
        state.write_reg8(TPM_DATA_FIFO, *byte);
    }
    state.write_reg8(TPM_STS, TPM_STS_GO);
    Ok(())
}
