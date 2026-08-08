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

use crate::hardware::tpm::constants::{TPM_DATA_FIFO, TPM_STS, TPM_STS_DATA_AVAIL, TPM_STS_READY};
use crate::hardware::tpm::state::TpmState;
use crate::hardware::tpm::types::TpmError;

pub fn receive_response_impl(state: &TpmState, buf: &mut [u8]) -> Result<usize, TpmError> {
    if !state.initialized {
        return Err(TpmError::NotPresent);
    }
    // CRB signals completion by clearing its start bit and leaves the reply in
    // a buffer, so there is no data-available bit here to wait on.
    if state.is_crb {
        return super::crb_receive(state, buf);
    }
    state.wait_for_status(TPM_STS_DATA_AVAIL, TPM_STS_DATA_AVAIL)?;
    let mut received = 0;
    while received < buf.len() {
        let sts = state.read_reg8(TPM_STS);
        if (sts & TPM_STS_DATA_AVAIL) == 0 {
            break;
        }
        buf[received] = state.read_reg8(TPM_DATA_FIFO);
        received += 1;
    }
    state.write_reg8(TPM_STS, TPM_STS_READY);
    Ok(received)
}
