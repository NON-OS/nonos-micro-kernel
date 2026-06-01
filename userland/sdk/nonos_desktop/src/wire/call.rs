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

use nonos_ipc::call;

use super::build::build_request;
use super::constants::HDR_LEN;

pub(crate) fn call_status(
    port: u32,
    magic: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
) -> Result<i32, &'static str> {
    let tx = build_request(magic, op, request_id, payload);
    let mut rx = vec![0u8; HDR_LEN + 4];
    let rc = call(port as u64, &tx, &mut rx);
    if rc <= 0 || (rc as usize) < HDR_LEN + 4 {
        return Err("ipc call failed");
    }
    Ok(i32::from_le_bytes(rx[HDR_LEN..HDR_LEN + 4].try_into().unwrap()))
}
