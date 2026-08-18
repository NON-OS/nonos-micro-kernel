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

use crate::wire::{read_u32, HDR_LEN};

pub const MAX_CHUNK: u32 = 65536;

pub struct VfsStream {
    pub(super) port: u32,
    pub(super) owner_pid: u32,
    pub(super) fd: u32,
    pub(super) rx: Vec<u8>,
}

impl VfsStream {
    pub fn open(owner_pid: u32, path: &[u8]) -> Result<VfsStream, &'static str> {
        if path.is_empty() || path.len() > 255 {
            return Err("vfs path invalid");
        }
        let port = super::resolve::vfs_port();
        let mut body = Vec::with_capacity(9 + path.len());
        body.extend_from_slice(&owner_pid.to_le_bytes());
        body.push(path.len() as u8);
        body.extend_from_slice(path);
        body.extend_from_slice(&0u32.to_le_bytes());
        let mut rx = vec![0u8; HDR_LEN + 4 + MAX_CHUNK as usize];
        let (status, total) = super::call::call(port, super::types::OP_OPEN, 2, &body, &mut rx)?;
        if status != 0 || total < HDR_LEN + 8 {
            return Err("vfs open failed");
        }
        let fd = read_u32(&rx, HDR_LEN + 4)?;
        Ok(VfsStream { port, owner_pid, fd, rx })
    }

    pub fn seek(&mut self, offset: u64) -> Result<u64, &'static str> {
        let at = i64::try_from(offset).map_err(|_| "vfs seek offset too large")?;
        let mut body = [0u8; 17];
        body[..4].copy_from_slice(&self.owner_pid.to_le_bytes());
        body[4..8].copy_from_slice(&self.fd.to_le_bytes());
        body[8] = super::types::SEEK_SET;
        body[9..17].copy_from_slice(&at.to_le_bytes());
        let (status, total) =
            super::call::call(self.port, super::types::OP_SEEK, 5, &body, &mut self.rx)?;
        if status != 0 || total < HDR_LEN + 12 {
            return Err("vfs seek failed");
        }
        let lo = read_u32(&self.rx, HDR_LEN + 4)? as u64;
        let hi = read_u32(&self.rx, HDR_LEN + 8)? as u64;
        Ok(lo | (hi << 32))
    }
}
