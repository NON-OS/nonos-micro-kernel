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

use super::proto::{Header, E_OK, HDR_LEN, IPC_PAYLOAD_MAX, OP_GET_SIZE};

const REPLY_TIMEOUT_MS: u64 = 500;

pub fn fetch_size(catalog_port: u32, index: u32) -> Option<u32> {
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let req = Header { op: OP_GET_SIZE, status: 0, index, offset: 0, payload_len: 0 };
    req.encode(&mut buf[..HDR_LEN]);
    let n = mk_ipc_call_timeout(
        catalog_port as u64,
        buf.as_ptr(),
        HDR_LEN,
        buf.as_mut_ptr(),
        buf.len(),
        REPLY_TIMEOUT_MS,
    );
    if n <= 0 || (n as usize) < HDR_LEN + 4 {
        return None;
    }
    let hdr = Header::decode(&buf[..HDR_LEN])?;
    if hdr.status != E_OK || hdr.op != OP_GET_SIZE || hdr.index != index {
        return None;
    }
    if (hdr.payload_len as usize) < 4 {
        return None;
    }
    Some(u32::from_le_bytes([buf[HDR_LEN], buf[HDR_LEN + 1], buf[HDR_LEN + 2], buf[HDR_LEN + 3]]))
}
