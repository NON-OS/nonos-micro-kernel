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

//! Ask the installer to load a capsule-store app by name. Only the name and the
//! cap ceiling travel here; the installer reads the artifacts from the store
//! itself, so the request stays small however large the capsule is. A missing
//! installer, a refusal and a short reply all come back as None, so a failed
//! launch is inert rather than a bad pid.

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{HDR_LEN, LOAD_TIMEOUT_MS, MAX_NAME, OP_LOAD_BY_NAME, REQUESTED_CAPS, SEQ};
use super::port::port;

/// Reply is seq(4) | status(4) | pid(4); nothing past the pid is read.
const PID_END: usize = HDR_LEN + 4;

pub fn load_by_name(name: &[u8]) -> Option<u32> {
    if name.is_empty() || name.len() > MAX_NAME {
        return None;
    }
    let tx = request(name);
    let mut rx = [0u8; 32];
    let rc = mk_ipc_call_timeout(
        port()? as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        LOAD_TIMEOUT_MS,
    );
    if rc <= 0 || (rc as usize) < PID_END {
        return None;
    }
    if i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]) != 0 {
        return None;
    }
    match u32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]) {
        0 => None,
        pid => Some(pid),
    }
}

/// Body after the header is requested_caps(8) | name_len(1) | name | args, with
/// no args: the Launchpad launches an app plainly, as a dock click does.
fn request(name: &[u8]) -> Vec<u8> {
    let mut tx = Vec::with_capacity(HDR_LEN + 9 + name.len());
    tx.extend_from_slice(&SEQ.to_le_bytes());
    tx.extend_from_slice(&OP_LOAD_BY_NAME.to_le_bytes());
    tx.extend_from_slice(&[0u8, 0u8]);
    tx.extend_from_slice(&REQUESTED_CAPS.to_le_bytes());
    tx.push(name.len() as u8);
    tx.extend_from_slice(name);
    tx
}
