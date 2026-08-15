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

use nonos_app_skeleton::clients::vfs::VfsStream;

pub const WINDOW: u32 = 1024 * 1024;

pub struct Source {
    stream: VfsStream,
    window: Vec<u8>,
    window_pos: u64,
}

impl Source {
    pub fn open(owner_pid: u32, path: &[u8]) -> Result<Source, &'static str> {
        let stream = VfsStream::open(owner_pid, path)?;
        Ok(Source { stream, window: Vec::new(), window_pos: 0 })
    }

    pub fn read_header(&mut self, max: u32) -> Result<Vec<u8>, &'static str> {
        self.stream.read_window(0, max)
    }

    pub fn bytes_at(&mut self, off: u64, len: u32) -> Result<&[u8], &'static str> {
        let end = off.checked_add(len as u64).ok_or("frame span overflows")?;
        let have = self.window_pos.saturating_add(self.window.len() as u64);
        if off < self.window_pos || end > have {
            self.refill(off, len)?;
        }
        let s = (off - self.window_pos) as usize;
        let e = s.checked_add(len as usize).ok_or("frame span overflows")?;
        self.window.get(s..e).ok_or("frame span outside window")
    }

    fn refill(&mut self, off: u64, len: u32) -> Result<(), &'static str> {
        let want = core::cmp::max(WINDOW, len);
        let buf = self.stream.read_window(off, want)?;
        if (buf.len() as u32) < len {
            return Err("short read at frame offset");
        }
        self.window = buf;
        self.window_pos = off;
        Ok(())
    }
}
