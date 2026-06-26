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

use super::consts::{response_code, READ_CMD, RC_SUBMIT_FAILED};
use crate::security::tpm_extend::submit_tpm_command;
use uefi::table::boot::BootServices;

pub fn nv_read_counter(bs: &BootServices) -> Result<u64, u32> {
    let mut resp = [0u8; 64];
    let n = match submit_tpm_command(bs, &READ_CMD, &mut resp) {
        Ok(n) => n,
        Err(_) => return Err(RC_SUBMIT_FAILED),
    };
    let rc = response_code(&resp);
    if rc != 0 {
        return Err(rc);
    }
    if n < 24 {
        return Err(RC_SUBMIT_FAILED);
    }
    Ok(u64::from_be_bytes([
        resp[16], resp[17], resp[18], resp[19], resp[20], resp[21], resp[22], resp[23],
    ]))
}
