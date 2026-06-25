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

use super::consts::{response_code, DEFINE_COUNTER_CMD, RC_NV_DEFINED, RC_SUBMIT_FAILED};
use crate::security::tpm_extend::submit_tpm_command;
use uefi::table::boot::BootServices;

pub fn nv_define_counter(bs: &BootServices) -> Result<(), u32> {
    let mut resp = [0u8; 64];
    if submit_tpm_command(bs, &DEFINE_COUNTER_CMD, &mut resp).is_err() {
        return Err(RC_SUBMIT_FAILED);
    }
    let rc = response_code(&resp);
    if rc == 0 || rc == RC_NV_DEFINED {
        Ok(())
    } else {
        Err(rc)
    }
}
