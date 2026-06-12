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

use super::file::File;
use super::frame::{BODY_OFF, OP_READ, OP_WRITE};
use super::session::call;
use crate::io::{Read, Result, Write};

const MAX_READ: usize = 65536;

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let want = buf.len().min(MAX_READ) as u32;
        let mut body = Vec::with_capacity(12);
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.extend_from_slice(&want.to_le_bytes());
        let rx = call(self.port, OP_READ, &body, MAX_READ)?;
        let data = &rx[BODY_OFF..];
        let n = data.len().min(buf.len());
        buf[..n].copy_from_slice(&data[..n]);
        Ok(n)
    }
}

impl Write for File {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        let mut body = Vec::with_capacity(8 + data.len());
        body.extend_from_slice(&self.pid.to_le_bytes());
        body.extend_from_slice(&self.fd.to_le_bytes());
        body.extend_from_slice(data);
        let rx = call(self.port, OP_WRITE, &body, 8)?;
        let wrote =
            u32::from_le_bytes([rx[BODY_OFF], rx[BODY_OFF + 1], rx[BODY_OFF + 2], rx[BODY_OFF + 3]]);
        Ok(wrote as usize)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
