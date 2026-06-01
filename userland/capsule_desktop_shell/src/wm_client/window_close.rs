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

const MAGIC: u32 = 0x4E57_4D50;
const VERSION: u16 = 1;
const HDR_LEN: usize = 20;
const STATUS_LEN: usize = 4;
const BODY_LEN: usize = 8;
const OP: u16 = 0x0003;
const REPLY_TIMEOUT_MS: u64 = 250;

pub fn window_close(port: u32, request_id: u32, window_id: u32) -> Result<(), &'static str> {
    let mut tx = [0u8; HDR_LEN + BODY_LEN];
    tx[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    tx[4..6].copy_from_slice(&VERSION.to_le_bytes());
    tx[6..8].copy_from_slice(&OP.to_le_bytes());
    tx[12..16].copy_from_slice(&request_id.to_le_bytes());
    tx[16..20].copy_from_slice(&(BODY_LEN as u32).to_le_bytes());
    tx[20..24].copy_from_slice(&window_id.to_le_bytes());
    let mut rx = [0u8; HDR_LEN + STATUS_LEN];
    let rc = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len(), REPLY_TIMEOUT_MS);
    if rc < (HDR_LEN + STATUS_LEN) as i64 {
        return Err("wm close failed");
    }
    let status = i32::from_le_bytes(rx[HDR_LEN..HDR_LEN + STATUS_LEN].try_into().unwrap());
    if status != 0 {
        return Err("wm rejected window_close");
    }
    Ok(())
}
