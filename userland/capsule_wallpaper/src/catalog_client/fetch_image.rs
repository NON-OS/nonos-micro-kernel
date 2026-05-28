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

use super::fetch_size::fetch_size;
use super::proto::{Header, CHUNK_MAX, E_OK, HDR_LEN, IPC_PAYLOAD_MAX, OP_GET_CHUNK};

const REPLY_TIMEOUT_MS: u64 = 500;
const MAX_IMAGE_BYTES: u32 = 2_000_000;
const MAX_CHUNKS: u32 = (MAX_IMAGE_BYTES / CHUNK_MAX as u32) + 2;

pub fn fetch_image(catalog_port: u32, index: u32) -> Option<Vec<u8>> {
    let size = fetch_size(catalog_port, index)?;
    if size == 0 || size > MAX_IMAGE_BYTES {
        return None;
    }
    let mut out: Vec<u8> = Vec::with_capacity(size as usize);
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let mut offset: u32 = 0;
    let mut iterations: u32 = 0;
    while offset < size {
        iterations += 1;
        if iterations > MAX_CHUNKS {
            return None;
        }
        let req = Header { op: OP_GET_CHUNK, status: 0, index, offset, payload_len: 0 };
        req.encode(&mut buf[..HDR_LEN]);
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
        let hdr = Header::decode(&buf[..HDR_LEN])?;
        if hdr.status != E_OK || hdr.op != OP_GET_CHUNK {
            return None;
        }
        if hdr.index != index || hdr.offset != offset {
            return None;
        }
        if hdr.payload_len as usize > CHUNK_MAX {
            return None;
        }
        let body_end = HDR_LEN + hdr.payload_len as usize;
        if body_end > n as usize {
            return None;
        }
        if hdr.payload_len == 0 {
            return None;
        }
        let new_offset = offset.checked_add(hdr.payload_len)?;
        if new_offset > size {
            return None;
        }
        out.extend_from_slice(&buf[HDR_LEN..body_end]);
        offset = new_offset;
    }
    if out.len() as u32 != size {
        return None;
    }
    Some(out)
}
