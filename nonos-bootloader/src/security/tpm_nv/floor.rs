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

use super::consts::{
    response_code, DEFINE_COUNTER_CMD, INCREMENT_CMD, READ_CMD, RC_NV_UNINITIALIZED,
};
use crate::security::tpm_extend::{submit_tpm_command, tpm_present};
use uefi::table::boot::BootServices;

fn rb_define() -> [u8; 45] {
    let mut cmd = DEFINE_COUNTER_CMD;
    cmd[34] = 0x20;
    cmd
}

fn rb_increment() -> [u8; 31] {
    let mut cmd = INCREMENT_CMD;
    cmd[13] = 0x20;
    cmd[17] = 0x20;
    cmd
}

fn rb_read() -> [u8; 35] {
    let mut cmd = READ_CMD;
    cmd[13] = 0x20;
    cmd[17] = 0x20;
    cmd
}

pub fn read_floor(bs: &BootServices) -> Option<u64> {
    if !tpm_present(bs) {
        return None;
    }
    let mut resp = [0u8; 64];
    let _ = submit_tpm_command(bs, &rb_define(), &mut resp);
    let n = submit_tpm_command(bs, &rb_read(), &mut resp).ok()?;
    let rc = response_code(&resp);
    if rc == RC_NV_UNINITIALIZED {
        return Some(0);
    }
    if rc != 0 || n < 24 {
        return None;
    }
    Some(u64::from_be_bytes([
        resp[16], resp[17], resp[18], resp[19], resp[20], resp[21], resp[22], resp[23],
    ]))
}

pub fn commit_floor(bs: &BootServices, target: u64) -> bool {
    let mut current = match read_floor(bs) {
        Some(value) => value,
        None => return false,
    };
    let mut guard = 0u32;
    while current < target {
        if guard >= 4096 {
            return false;
        }
        let mut resp = [0u8; 32];
        if submit_tpm_command(bs, &rb_increment(), &mut resp).is_err() || response_code(&resp) != 0 {
            return false;
        }
        current += 1;
        guard += 1;
    }
    true
}
