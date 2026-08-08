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

use alloc::vec::Vec;

use nonos_app_skeleton::discover::lookup_service;
use nonos_libc::mk_ipc_call_timeout;

pub(super) const EAGAIN: i32 = -11;

// A reply the installer never sends in any status it defines, so it can
// stand for "the frame itself was wrong" without shadowing a real status.
pub(super) const EPROTO: i32 = -71;

// Installer wire: seq(4) | op(2) | pad(2) | body, replying seq(4) |
// status(4) | payload. Query and commit both re-read and re-verify a
// multi-megabyte package before answering, so they need the same wide
// budget the load path takes rather than the 5s default.
const SEQ: u32 = 1;
const PKG_TIMEOUT_MS: u64 = 30_000;

// Returns the total reply length, header included, so a caller that wants
// the payload slices from byte 8. No reply at all is a not-ready installer,
// while a frame too short to hold a status field is a protocol fault.
pub(super) fn call(op: u16, body: &[u8], rx: &mut [u8]) -> Result<usize, i32> {
    let port = lookup_service(b"installer").map(|p| p.port).ok_or(EAGAIN)?;
    let mut tx = Vec::with_capacity(8 + body.len());
    tx.extend_from_slice(&SEQ.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&[0u8, 0u8]);
    tx.extend_from_slice(body);
    let rc = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        PKG_TIMEOUT_MS,
    );
    if rc < 0 {
        return Err(EAGAIN);
    }
    if rc < 8 {
        return Err(EPROTO);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    Ok((rc as usize).min(rx.len()))
}
