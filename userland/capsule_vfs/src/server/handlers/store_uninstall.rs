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

//! Payload: caller pid(4) | path_len(1) | path. Drops one artifact from the
//! RAM tree and from the on-device TOC in a single step, since leaving either
//! half behind resurrects a removed app on the next boot. Idempotent: a path
//! that is already gone from both is a success, so a retried uninstall after a
//! partial failure converges.

use alloc::vec::Vec;

use super::artifact_path::split_artifact;
use super::installer_gate::require_installer;
use super::util::{map_store_err, split_caller};
use crate::protocol::{encode_response, Request, EINVAL, OP_STORE_UNINSTALL};
use crate::store::{Store, StoreError};

pub fn store_uninstall(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let status = match drop_artifact(store, req, sender_pid) {
        Ok(()) => 0,
        Err(s) => s,
    };
    encode_response(OP_STORE_UNINSTALL, req.flags, req.request_id, status, &[])
}

fn drop_artifact(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Result<(), i32> {
    require_installer(sender_pid)?;
    let (_pid, rest) = split_caller(req.payload, sender_pid)?;
    let (path, tail) = split_artifact(rest)?;
    if !tail.is_empty() {
        return Err(EINVAL);
    }
    match store.unlink(&path) {
        Ok(()) | Err(StoreError::NotFound) => {}
        Err(e) => return Err(map_store_err(e)),
    }
    crate::blk::store_remove::remove(&path).map_err(|_| EINVAL)
}
