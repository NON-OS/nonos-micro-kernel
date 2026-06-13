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

mod ops;

use crate::io::Result;
use crate::net::proto::{call, port, BODY_OFF, OP_CLOSE, OP_SOCKET};

pub(crate) const KIND_STREAM: u16 = 1;
pub(crate) const KIND_DGRAM: u16 = 2;

pub(crate) struct Socket {
    port: u32,
    handle: u32,
}

impl Socket {
    pub(crate) fn open(kind: u16) -> Result<Self> {
        let p = port()?;
        let mut body = [0u8; 4];
        body[0..2].copy_from_slice(&4u16.to_le_bytes());
        body[2..4].copy_from_slice(&kind.to_le_bytes());
        let rx = call(p, OP_SOCKET, &body, 8)?;
        Ok(Self { port: p, handle: read_u32(&rx, BODY_OFF) })
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        let _ = call(self.port, OP_CLOSE, &self.handle.to_le_bytes(), 0);
    }
}

pub(crate) fn read_u32(buf: &[u8], off: usize) -> u32 {
    if buf.len() < off + 4 {
        return 0;
    }
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
