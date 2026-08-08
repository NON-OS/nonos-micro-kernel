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

//! Transport for the installer's package ops. These carry a body, unlike the
//! listing calls in `call.rs`, and they wait out the full dual-signature verify
//! the installer runs before it answers.

use alloc::vec::Vec;

use nonos_libc::mk_ipc_call_timeout;

use super::constants::{HDR_LEN, LOAD_TIMEOUT_MS, SEQ};
use super::port::port;

/// Local stand-in when the installer cannot be reached or replies short.
pub(super) const ERR_NOT_READY: i32 = -11;
/// Local stand-in for a path the request cannot even carry.
pub(super) const ERR_INVALID: i32 = -22;

pub(super) fn call(op: u16, body: &[u8], rx: &mut [u8]) -> Result<usize, i32> {
    let port = port().ok_or(ERR_NOT_READY)? as u64;
    let tx = frame(op, body);
    let rc = mk_ipc_call_timeout(
        port,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        LOAD_TIMEOUT_MS,
    );
    if rc <= 0 || (rc as usize) < HDR_LEN {
        return Err(ERR_NOT_READY);
    }
    let status = i32::from_le_bytes([rx[4], rx[5], rx[6], rx[7]]);
    if status != 0 {
        return Err(status);
    }
    Ok((rc as usize).min(rx.len()))
}

pub(super) fn len_prefixed(v: &[u8]) -> Vec<u8> {
    let mut b = Vec::with_capacity(2 + v.len());
    b.extend_from_slice(&(v.len() as u16).to_le_bytes());
    b.extend_from_slice(v);
    b
}

/// Header is seq(4) | op(2) | pad(2), the same shape `frame::build` emits.
fn frame(op: u16, body: &[u8]) -> Vec<u8> {
    let mut tx = Vec::with_capacity(HDR_LEN + body.len());
    tx.extend_from_slice(&SEQ.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&[0u8, 0u8]);
    tx.extend_from_slice(body);
    tx
}
