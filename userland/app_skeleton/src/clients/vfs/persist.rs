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

use crate::wire::HDR_LEN;

pub fn persist(owner_pid: u32, path: &[u8]) -> Result<(), &'static str> {
    if path.is_empty() || path.len() > 255 {
        return Err("vfs path invalid");
    }
    let port = super::resolve::vfs_port();
    let mut body = Vec::with_capacity(5 + path.len());
    body.extend_from_slice(&owner_pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    let mut rx = vec![0u8; HDR_LEN + 8];
    let (status, _) = super::call::call(port, super::types::OP_STORE_PERSIST, 17, &body, &mut rx)?;
    if status != 0 {
        return Err(super::errmsg::errmsg(status));
    }
    Ok(())
}
