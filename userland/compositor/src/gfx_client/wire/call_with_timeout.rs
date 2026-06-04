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

use nonos_libc::mk_ipc_call_timeout;

pub fn call_with_timeout(
    gfx_port: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    rx: &mut Vec<u8>,
    timeout_ms: u64,
) -> Result<usize, &'static str> {
    let mut tx = Vec::with_capacity(super::NVGP_HDR_LEN + payload.len());
    super::build_request(&mut tx, op, request_id, payload);
    let rc = mk_ipc_call_timeout(
        gfx_port as u64,
        tx.as_ptr(),
        tx.len(),
        rx.as_mut_ptr(),
        rx.len(),
        timeout_ms,
    );
    if rc <= 0 {
        return Err("gfx ipc call failed");
    }
    Ok(rc as usize)
}
