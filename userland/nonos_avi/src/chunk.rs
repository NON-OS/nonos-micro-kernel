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

use crate::bytes::{fourcc_at, u32_at};

pub struct Chunk<'a> {
    pub id: [u8; 4],
    pub data: &'a [u8],
}

pub struct ChunkIter<'a> {
    buf: &'a [u8],
    pos: usize,
}

pub fn chunks(buf: &[u8]) -> ChunkIter<'_> {
    ChunkIter { buf, pos: 0 }
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Chunk<'a>> {
        let id = fourcc_at(self.buf, self.pos).ok()?;
        let size = u32_at(self.buf, self.pos.checked_add(4)?).ok()? as usize;
        let start = self.pos.checked_add(8)?;
        let end = start.checked_add(size)?;
        if end > self.buf.len() {
            return None;
        }
        self.pos = end.checked_add(end & 1)?;
        Some(Chunk { id, data: &self.buf[start..end] })
    }
}
