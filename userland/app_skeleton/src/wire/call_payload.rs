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

use nonos_libc::mk_ipc_call;

use super::builder::build_request;
use super::constants::HDR_LEN;
use super::read_i32::read_i32;

pub fn call_payload(
    port: u32,
    magic: u32,
    op: u16,
    request_id: u32,
    payload: &[u8],
    rx: &mut [u8],
) -> Result<usize, &'static str> {
    let tx = build_request(magic, op, request_id, payload);
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    if rc <= 0 || (rc as usize) < HDR_LEN + 4 {
        return Err("ipc call failed");
    }
    let status = read_i32(rx, HDR_LEN)?;
    if status != 0 {
        return Err("service returned error");
    }
    Ok(rc as usize)
}
