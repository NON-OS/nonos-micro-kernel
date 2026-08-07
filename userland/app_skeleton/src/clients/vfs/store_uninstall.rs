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

/// Drop a capsule artifact from both the RAM tree and the on-device store. The
/// path must be `/capsules/<name><ext>`; an already-absent path succeeds. The
/// server errno is returned verbatim; a local encoding or transport failure
/// maps to a fixed negative code.
pub fn store_uninstall(path: &[u8]) -> Result<(), i32> {
    if path.is_empty() || path.len() > 255 {
        return Err(ERR_INVALID);
    }
    let port = super::resolve::vfs_port();
    let mut body = Vec::with_capacity(5 + path.len());
    body.extend_from_slice(&mk_getpid().to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    let mut rx = vec![0u8; HDR_LEN + 8];
    let (status, _) = super::call::call(port, super::types::OP_STORE_UNINSTALL, 21, &body, &mut rx)
        .map_err(|_| ERR_TRANSPORT)?;
    if status != 0 {
        return Err(status);
    }
    Ok(())
}
