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

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

use super::build::build;
use super::constants::HDR_LEN;
use super::u32_at::u32_at;

pub fn call(
    port: u32,
    magic: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    body: &mut [u8],
) -> Result<i32, &'static str> {
    let mut tx = Vec::with_capacity(HDR_LEN + payload.len());
    build(&mut tx, magic, op, request_id, payload);
    let mut rx = vec![0u8; HDR_LEN + 4 + body.len()];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (HDR_LEN + 4 + body.len()) as i64 || u32_at(&rx, 0)? != magic {
        return Err("service call failed");
    }
    body.copy_from_slice(&rx[HDR_LEN + 4..HDR_LEN + 4 + body.len()]);
    Ok(i32::from_le_bytes(rx[HDR_LEN..HDR_LEN + 4].try_into().map_err(|_| "status missing")?))
}
