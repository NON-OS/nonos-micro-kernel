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

use super::proto::{Header, CHUNK_MAX, E_OK, HDR_LEN, OP_GET_CHUNK};

const REPLY_TIMEOUT_MS: u64 = 500;

pub fn fetch_chunk(catalog_port: u32, index: u32, offset: u32, buf: &mut [u8]) -> Option<u32> {
    let req = Header { op: OP_GET_CHUNK, status: 0, index, offset, payload_len: 0 };
    req.encode(buf.get_mut(..HDR_LEN)?);
    let n = mk_ipc_call_timeout(
        catalog_port as u64,
        buf.as_ptr(),
        HDR_LEN,
        buf.as_mut_ptr(),
        buf.len(),
        REPLY_TIMEOUT_MS,
    );
    if n <= 0 || (n as usize) < HDR_LEN {
        return None;
    }
    let hdr = Header::decode(buf.get(..HDR_LEN)?)?;
    if hdr.status != E_OK || hdr.op != OP_GET_CHUNK {
        return None;
    }
    if hdr.index != index || hdr.offset != offset {
        return None;
    }
    if hdr.payload_len == 0 || hdr.payload_len as usize > CHUNK_MAX {
        return None;
    }
    let body_end = HDR_LEN.checked_add(hdr.payload_len as usize)?;
    if body_end > n as usize {
        return None;
    }
    Some(hdr.payload_len)
}
