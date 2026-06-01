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

use super::cache::STRING_CAP;

#[derive(Clone, Copy)]
pub struct EditBuffer {
    pub bytes: [u8; STRING_CAP],
    pub len: usize,
}

impl EditBuffer {
    pub const fn empty() -> Self {
        Self { bytes: [0u8; STRING_CAP], len: 0 }
    }

    pub fn push(&mut self, b: u8) -> bool {
        if self.len >= STRING_CAP {
            return false;
        }
        self.bytes[self.len] = b;
        self.len += 1;
        true
    }

    pub fn pop(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.len -= 1;
        self.bytes[self.len] = 0;
        true
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
