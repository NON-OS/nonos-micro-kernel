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

use alloc::{vec, vec::Vec};

use nonos_libc::mk_getpid;

use crate::wire::HDR_LEN;

const ERR_INVALID: i32 = -22;
const ERR_TRANSPORT: i32 = -5;

// Matches write_file: the vfs caps one request's data at MAX_DATA_BYTES and the
// whole request at MAX_PAYLOAD_BYTES, so a multi-megabyte artifact travels as a
// run of chunks. Here the offset lives in the body rather than on an open fd,
// because the server must know which chunk is last to persist the file once.
const CHUNK: usize = 60 * 1024;

/// Place a capsule artifact under `/capsules/` and commit it to the on-device
/// store. The server rejects any path that is not `/capsules/<name><ext>` for a
/// known artifact extension. The server errno is returned verbatim; a local
/// encoding or transport failure maps to a fixed negative code.
pub fn store_install(path: &[u8], bytes: &[u8]) -> Result<(), i32> {
    if path.is_empty() || path.len() > 255 {
        return Err(ERR_INVALID);
    }
    let port = super::resolve::vfs_port();
    let pid = mk_getpid();
    let mut rx = vec![0u8; HDR_LEN + 8];
    let mut sent = 0usize;
    loop {
        let end = (sent + CHUNK).min(bytes.len());
        let last = if end == bytes.len() { super::types::STORE_INSTALL_FINAL } else { 0 };
        let body = frame(pid, path, last, sent as u32, &bytes[sent..end]);
        let (status, _) =
            super::call::call(port, super::types::OP_STORE_INSTALL, 20, &body, &mut rx)
                .map_err(|_| ERR_TRANSPORT)?;
        if status != 0 {
            return Err(status);
        }
        sent = end;
        if last != 0 {
            return Ok(());
        }
    }
}

fn frame(pid: u32, path: &[u8], flags: u8, offset: u32, chunk: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(10 + path.len() + chunk.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    body.push(flags);
    body.extend_from_slice(&offset.to_le_bytes());
    body.extend_from_slice(chunk);
    body
}
