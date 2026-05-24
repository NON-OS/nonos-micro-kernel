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

const MAGIC: u32 = 0x4E4D_4B54;
const VERSION: u16 = 1;
const HDR_LEN: usize = 20;
const STATUS_LEN: usize = 4;
const OP_HEALTHCHECK: u16 = 0x0006;

pub fn healthcheck(port: u32, request_id: u32) -> Result<i32, &'static str> {
    if port == 0 {
        return Ok(0);
    }
    let mut tx = Vec::with_capacity(HDR_LEN);
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&OP_HEALTHCHECK.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc < (HDR_LEN + STATUS_LEN) as i64 {
        return Err("market call failed");
    }
    let bytes: [u8; 4] = match rx[HDR_LEN..HDR_LEN + STATUS_LEN].try_into() {
        Ok(b) => b,
        Err(_) => return Err("market short response"),
    };
    Ok(i32::from_le_bytes(bytes))
}
