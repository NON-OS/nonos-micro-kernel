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

use crate::discover::lookup_service;
use crate::wire::HDR_LEN;

pub fn rename(owner_pid: u32, old: &[u8], new: &[u8]) -> Result<(), &'static str> {
    if old.is_empty() || old.len() > 255 || new.is_empty() || new.len() > 255 {
        return Err("vfs path invalid");
    }
    let peer = lookup_service(super::types::NAME).ok_or("vfs unavailable")?;
    let mut body = Vec::with_capacity(6 + old.len() + new.len());
    body.extend_from_slice(&owner_pid.to_le_bytes());
    body.push(old.len() as u8);
    body.extend_from_slice(old);
    body.push(new.len() as u8);
    body.extend_from_slice(new);
    let mut rx = vec![0u8; HDR_LEN + 8];
    let (status, _) = super::call::call(peer.port, super::types::OP_RENAME, 10, &body, &mut rx)?;
    if status != 0 {
        return Err(super::errmsg::errmsg(status));
    }
    Ok(())
}
