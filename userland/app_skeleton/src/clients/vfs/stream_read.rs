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

use super::stream::{VfsStream, MAX_CHUNK};
use crate::wire::HDR_LEN;

impl VfsStream {
    pub fn read_into(&mut self, want: u32, out: &mut Vec<u8>) -> Result<usize, &'static str> {
        let want = core::cmp::min(want, MAX_CHUNK);
        let mut body = [0u8; 12];
        body[..4].copy_from_slice(&self.owner_pid.to_le_bytes());
        body[4..8].copy_from_slice(&self.fd.to_le_bytes());
        body[8..12].copy_from_slice(&want.to_le_bytes());
        let (status, total) =
            super::call::call(self.port, super::types::OP_READ, 3, &body, &mut self.rx)?;
        if status != 0 || total < HDR_LEN + 4 {
            return Err("vfs read failed");
        }
        out.extend_from_slice(&self.rx[HDR_LEN + 4..total]);
        Ok(total - (HDR_LEN + 4))
    }

    pub fn read_window(&mut self, offset: u64, len: u32) -> Result<Vec<u8>, &'static str> {
        self.seek(offset)?;
        let mut out = Vec::new();
        out.try_reserve(len as usize).map_err(|_| "vfs window too large")?;
        while (out.len() as u32) < len {
            let want = len - out.len() as u32;
            if self.read_into(want, &mut out)? == 0 {
                break;
            }
        }
        Ok(out)
    }
}

impl Drop for VfsStream {
    fn drop(&mut self) {
        let mut body = [0u8; 8];
        body[..4].copy_from_slice(&self.owner_pid.to_le_bytes());
        body[4..8].copy_from_slice(&self.fd.to_le_bytes());
        let _ = super::call::call(self.port, super::types::OP_CLOSE, 4, &body, &mut self.rx);
    }
}
