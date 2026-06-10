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

pub fn send_read_public(state: &TpmState, cmd: &[u8]) -> Result<(), &'static str> {
    state.write_reg8(TPM_STS, TPM_STS_READY);
    for _ in 0..10000 {
        if (state.read_reg8(TPM_STS) & TPM_STS_READY) != 0 {
            break;
        }
        core::hint::spin_loop();
    }
    for byte in cmd {
        state.write_reg8(TPM_DATA_FIFO, *byte);
    }
    state.write_reg8(TPM_STS, TPM_STS_GO);
    Ok(())
}
