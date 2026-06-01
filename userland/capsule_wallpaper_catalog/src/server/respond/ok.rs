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

use crate::protocol::{Header, E_BAD_LEN, E_OK, HDR_LEN, IPC_PAYLOAD_MAX};

use super::super::reply;
use super::err::err;

const PAYLOAD_CAP: usize = IPC_PAYLOAD_MAX - HDR_LEN;

pub fn ok(pid: u32, op: u16, index: u32, offset: u32, payload: &[u8]) {
    if payload.len() > PAYLOAD_CAP {
        err(pid, op, index, E_BAD_LEN);
        return;
    }
    let hdr = Header {
        op,
        status: E_OK,
        index,
        offset,
        payload_len: payload.len() as u32,
    };
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    hdr.encode(&mut buf[..HDR_LEN]);
    buf[HDR_LEN..HDR_LEN + payload.len()].copy_from_slice(payload);
    reply::send(pid, &buf[..HDR_LEN + payload.len()]);
}
