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

// File seek over the VFS: OP_SEEK carries the whence and offset and returns
// the new absolute position; tell is the zero-offset current-position form.

use super::File;
use crate::io::{self, SeekFrom};
use crate::sys::fs::nonos::transport::{BODY_OFF, OP_SEEK, SEEK_CUR, SEEK_END, SEEK_SET, call, err};
use crate::vec::Vec;

impl File {
    fn seek_raw(&self, whence: u8, offset: i64) -> io::Result<u64> {
        let mut body = Vec::with_capacity(21);
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.push(whence);
        body.extend_from_slice(&offset.to_le_bytes());
        let rx = call(self.port, OP_SEEK, &body, 8)?;
        let b = &rx[BODY_OFF..];
        if b.len() < 8 {
            return Err(err("short seek reply"));
        }
        Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        let (whence, offset) = match pos {
            SeekFrom::Start(n) => {
                let n = i64::try_from(n).map_err(|_| err("seek offset too large"))?;
                (SEEK_SET, n)
            }
            SeekFrom::Current(n) => (SEEK_CUR, n),
            SeekFrom::End(n) => (SEEK_END, n),
        };
        self.seek_raw(whence, offset)
    }

    pub fn tell(&self) -> io::Result<u64> {
        self.seek_raw(SEEK_CUR, 0)
    }
}
