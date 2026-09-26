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


//! The argument cursor.

pub struct Args<'a> {
    at: usize,
    body: &'a [u8],
}

impl<'a> Args<'a> {
    pub fn new(body: &'a [u8]) -> Args<'a> {
        Args { at: 0, body }
    }

    pub fn u32(&mut self) -> Option<u32> {
        let w = self.body.get(self.at..self.at + 4)?;
        self.at += 4;
        Some(u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
    }

    /// The bytes without the terminator. The field on the wire is padded
    /// to a word and the cursor skips the padding, never the caller.
    pub fn string(&mut self) -> Option<&'a [u8]> {
        let len = self.u32()? as usize;
        if len == 0 {
            return Some(&[]);
        }
        let text = self.body.get(self.at..self.at + len - 1)?;
        self.at += (len + 3) & !3;
        Some(text)
    }
}
