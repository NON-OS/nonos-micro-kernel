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

//! Send one request and hand back the reply length only when the installer
//! answered with a success status. A missing service, a timeout, a short reply
//! and a failed status all become None so callers never read a half-formed
//! buffer.

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{HDR_LEN, TIMEOUT_MS};
use super::frame::build;
use super::port::port;

pub(super) fn call(op: u16, rx: &mut [u8]) -> Option<usize> {
    let tx = build(op);
    let rc = mk_ipc_call_timeout(
        port()? as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if rc <= 0 || (rc as usize) < HDR_LEN {
        return None;
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return None;
    }
    Some(rc as usize)
}
