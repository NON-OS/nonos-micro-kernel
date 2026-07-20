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

// File writes over the VFS: OP_WRITE returns the accepted byte count, plus
// the vectored adapter and the flush/sync no-ops (the store commits each
// write, so there is nothing buffered to force out).

use super::File;
use crate::io::{self, IoSlice};
use crate::sys::fs::nonos::transport::{BODY_OFF, OP_WRITE, call, read_u32};
use crate::vec::Vec;

impl File {
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let mut body = Vec::with_capacity(8 + buf.len());
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.extend_from_slice(buf);
        let rx = call(self.port, OP_WRITE, &body, 8)?;
        Ok(read_u32(&rx, BODY_OFF) as usize)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.write(b);
            }
        }
        Ok(0)
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn fsync(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn datasync(&self) -> io::Result<()> {
        Ok(())
    }
}
