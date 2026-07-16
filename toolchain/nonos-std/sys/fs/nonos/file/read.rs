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

// File reads over the VFS: OP_READ up to 64 KiB a call, plus the BorrowedCursor
// and vectored adapters std's Read impl drives through this one primitive.

use super::File;
use crate::io::{self, BorrowedCursor, IoSliceMut};
use crate::sys::fs::nonos::transport::{BODY_OFF, OP_READ, call};
use crate::vec::Vec;

impl File {
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let want = buf.len().min(65536) as u32;
        let mut body = Vec::with_capacity(12);
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.extend_from_slice(&want.to_le_bytes());
        let rx = call(self.port, OP_READ, &body, 65536)?;
        let data = &rx[BODY_OFF..];
        let n = data.len().min(buf.len());
        buf[..n].copy_from_slice(&data[..n]);
        Ok(n)
    }

    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> io::Result<()> {
        let mut tmp = [0u8; 4096];
        let n = self.read(&mut tmp)?;
        cursor.append(&tmp[..n]);
        Ok(())
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.read(b);
            }
        }
        Ok(0)
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }
}
