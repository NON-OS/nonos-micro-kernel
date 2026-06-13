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

use super::frame::{BODY_OFF, OP_STAT};
use super::session::{call, connect, path_body};
use crate::io::{Error, ErrorKind, Result};

#[derive(Clone, Copy, Debug)]
pub struct Metadata {
    size: u64,
    is_dir: bool,
}

impl Metadata {
    pub fn len(&self) -> u64 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir
    }
}

pub fn metadata(path: &[u8]) -> Result<Metadata> {
    let vfs = connect()?;
    let rx = call(vfs.port, OP_STAT, &path_body(vfs.pid, path)?, 16)?;
    let body = &rx[BODY_OFF..];
    if body.len() < 12 {
        return Err(Error::new(ErrorKind::Other, "short stat reply"));
    }
    let size = u64::from_le_bytes([
        body[0], body[1], body[2], body[3], body[4], body[5], body[6], body[7],
    ]);
    let flags = u32::from_le_bytes([body[8], body[9], body[10], body[11]]);
    Ok(Metadata { size, is_dir: flags & 1 != 0 })
}
