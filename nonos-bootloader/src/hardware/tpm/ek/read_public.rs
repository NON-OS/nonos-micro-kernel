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
use super::receive::receive_read_public;
use super::send::send_read_public;
use crate::hardware::tpm::state::TpmState;
use alloc::vec::Vec;

const EK_HANDLE: u32 = 0x8101_0001;

pub fn get_ek_public_impl(state: &TpmState) -> Result<Vec<u8>, &'static str> {
    if !state.initialized {
        return Err("TPM not initialized");
    }
    state.request_locality().map_err(|_| "locality request failed")?;
    let mut cmd = [0u8; 14];
    cmd[0..2].copy_from_slice(&0x8001u16.to_be_bytes());
    cmd[2..6].copy_from_slice(&14u32.to_be_bytes());
    cmd[6..10].copy_from_slice(&0x0000_0173u32.to_be_bytes());
    cmd[10..14].copy_from_slice(&EK_HANDLE.to_be_bytes());
    send_read_public(state, &cmd)?;
    let response = receive_read_public(state)?;
    state.release_locality();
    Ok(response)
}
