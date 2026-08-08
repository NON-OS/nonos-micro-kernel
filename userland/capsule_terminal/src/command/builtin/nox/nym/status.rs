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

use nonos_libc::mk_ipc_call_timeout;

use super::wire::{header, HDR_LEN, TIMEOUT_MS};

/// Call one status op and hand back the reply payload.
///
/// A zero-length answer is not an error here: `healthcheck` carries no body,
/// so the caller distinguishes "reachable" from "has data" by what it asked
/// for rather than by the length.
pub fn status(port: u32, op: u16, payload_len: usize) -> Option<[u8; 32]> {
    let tx = header(op);
    let mut rx = [0u8; HDR_LEN + 32];
    let want = HDR_LEN + payload_len;
    let n = mk_ipc_call_timeout(
        port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        TIMEOUT_MS,
    );
    if n < want as i64 {
        return None;
    }
    if u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return None;
    }
    let mut out = [0u8; 32];
    out[..payload_len].copy_from_slice(&rx[HDR_LEN..want]);
    Some(out)
}
