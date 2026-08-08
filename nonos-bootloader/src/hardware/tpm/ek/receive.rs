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
use crate::hardware::tpm::state::TpmState;
use alloc::vec;
use alloc::vec::Vec;

/// Room for an RSA-2048 EK public area and then some.
const RESPONSE_CAPACITY: usize = 512;

/// A TPM 2.0 response header: tag, size, return code.
const HEADER_LEN: usize = 10;

/// Collect the ReadPublic response.
///
/// Reads through the shared transport, which picks FIFO or CRB from the
/// detected interface. This drained `TPM_DATA_FIFO` by hand, so on a CRB part
/// it collected nothing and reported a malformed reply for what was really a
/// driver reading the wrong registers.
///
/// A non-zero return code is its own failure. The likeliest one here is not a
/// broken TPM but an unprovisioned one: with no persistent object at the EK
/// handle the part answers `TPM_RC_HANDLE`, which is a true answer about the
/// machine rather than a transport fault.
pub fn receive_read_public(state: &TpmState) -> Result<Vec<u8>, &'static str> {
    let mut response = vec![0u8; RESPONSE_CAPACITY];
    let received = state.receive_response(&mut response).map_err(|_| "TPM response read failed")?;

    if received < HEADER_LEN {
        return Err("invalid TPM response");
    }
    response.truncate(received);

    let rc = u32::from_be_bytes([response[6], response[7], response[8], response[9]]);
    if rc != 0 {
        return Err("TPM command failed");
    }
    Ok(response)
}
