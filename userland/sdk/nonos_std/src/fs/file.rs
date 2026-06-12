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

use super::frame::{BODY_OFF, OP_CLOSE, OP_OPEN, O_CREATE, O_TRUNC};
use super::session::{call, connect};
use crate::io::{Error, ErrorKind, Result};

pub struct File {
    pub(super) port: u32,
    pub(super) pid: u32,
    pub(super) fd: u32,
}

impl File {
    pub fn open(path: &[u8]) -> Result<Self> {
        open_with(path, 0)
    }

    pub fn create(path: &[u8]) -> Result<Self> {
        open_with(path, O_CREATE | O_TRUNC)
    }
}

fn open_with(path: &[u8], flags: u32) -> Result<File> {
    if path.is_empty() || path.len() > 255 {
        return Err(Error::new(ErrorKind::InvalidInput, "bad path"));
    }
    let vfs = connect()?;
    let mut body = Vec::with_capacity(9 + path.len());
    body.extend_from_slice(&vfs.pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    body.extend_from_slice(&flags.to_le_bytes());
    let rx = call(vfs.port, OP_OPEN, &body, 8)?;
    let fd = u32::from_le_bytes([rx[BODY_OFF], rx[BODY_OFF + 1], rx[BODY_OFF + 2], rx[BODY_OFF + 3]]);
    Ok(File { port: vfs.port, pid: vfs.pid, fd })
}

impl Drop for File {
    fn drop(&mut self) {
        let mut body = [0u8; 8];
        body[..4].copy_from_slice(&self.pid.to_le_bytes());
        body[4..8].copy_from_slice(&self.fd.to_le_bytes());
        let _ = call(self.port, OP_CLOSE, &body, 0);
    }
}
