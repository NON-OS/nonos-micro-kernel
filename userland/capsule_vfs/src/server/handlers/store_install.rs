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

//! Payload: caller pid(4) | path_len(1) | path | flags(1) | offset(4) | bytes.
//! One artifact is sent as a run of chunks because a request cannot exceed
//! MAX_PAYLOAD_BYTES; the server appends at the offset the client states and
//! hands the assembled file to the on-device store when FINAL arrives. This is
//! the only write path allowed into the read-only /capsules tree, which is why
//! `split_artifact` — not `is_read_only` — is what stands between a caller and
//! that tree.

use alloc::vec::Vec;

use super::artifact_path::split_artifact;
use super::util::{map_store_err, split_caller};
use crate::protocol::{
    encode_response, Request, EINVAL, EMSGSIZE, ENOENT, MAX_DATA_BYTES, OP_STORE_INSTALL,
    STORE_INSTALL_FINAL,
};
use crate::store::Store;

const HEAD_LEN: usize = 5;

pub fn store_install(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let status = match place(store, req, sender_pid) {
        Ok(()) => 0,
        Err(s) => s,
    };
    encode_response(OP_STORE_INSTALL, req.flags, req.request_id, status, &[])
}

fn place(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Result<(), i32> {
    let (_pid, rest) = split_caller(req.payload, sender_pid)?;
    let (path, tail) = split_artifact(rest)?;
    if tail.len() < HEAD_LEN {
        return Err(EINVAL);
    }
    let flags = tail[0];
    let offset = u32::from_le_bytes([tail[1], tail[2], tail[3], tail[4]]) as usize;
    let data = &tail[HEAD_LEN..];
    if data.len() > MAX_DATA_BYTES as usize {
        return Err(EMSGSIZE);
    }
    store.install_bytes(&path, offset, data).map_err(map_store_err)?;
    if flags & STORE_INSTALL_FINAL == 0 {
        return Ok(());
    }
    let whole = store.bytes_of(&path).ok_or(ENOENT)?;
    crate::blk::store_write::append(&path, &whole).map_err(|_| EINVAL)
}
