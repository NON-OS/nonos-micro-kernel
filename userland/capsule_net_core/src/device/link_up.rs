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

use super::mac::next_rid;
use crate::protocol::header::{parse_response, write_request};
use crate::protocol::ops::{HDR_LEN, OP_LINK_STATUS};
use nonos_libc::mk_ipc_call;

pub fn link_up(port: u32) -> Option<bool> {
    let mut req = [0u8; HDR_LEN];
    write_request(&mut req, OP_LINK_STATUS, next_rid(), 0)?;
    let mut resp = [0u8; HDR_LEN + 5];
    let n = mk_ipc_call(port as u64, req.as_ptr(), HDR_LEN, resp.as_mut_ptr(), resp.len());
    if n < 0 || n as usize > resp.len() {
        return None;
    }
    let view = &resp[..n as usize];
    let (op, _, plen) = parse_response(view)?;
    if op != OP_LINK_STATUS || plen != 5 || view.len() < HDR_LEN + 5 {
        return None;
    }
    let status = i32::from_le_bytes([
        view[HDR_LEN],
        view[HDR_LEN + 1],
        view[HDR_LEN + 2],
        view[HDR_LEN + 3],
    ]);
    if status != 0 {
        return None;
    }
    Some(view[HDR_LEN + 4] != 0)
}
