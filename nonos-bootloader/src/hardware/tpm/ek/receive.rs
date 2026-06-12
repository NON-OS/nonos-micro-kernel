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

extern crate alloc;
use crate::hardware::tpm::constants::{TPM_DATA_FIFO, TPM_STS, TPM_STS_DATA_AVAIL};
use crate::hardware::tpm::state::TpmState;
use alloc::vec::Vec;

pub fn receive_read_public(state: &TpmState) -> Result<Vec<u8>, &'static str> {
    for _ in 0..10000 {
        if (state.read_reg8(TPM_STS) & TPM_STS_DATA_AVAIL) != 0 {
            break;
        }
        core::hint::spin_loop();
    }
    let mut response = Vec::with_capacity(512);
    for _ in 0..512 {
        if (state.read_reg8(TPM_STS) & TPM_STS_DATA_AVAIL) == 0 {
            break;
        }
        response.push(state.read_reg8(TPM_DATA_FIFO));
    }
    if response.len() < 10 {
        return Err("invalid TPM response");
    }
    let rc = u32::from_be_bytes([response[6], response[7], response[8], response[9]]);
    if rc != 0 {
        return Err("TPM command failed");
    }
    Ok(response)
}
