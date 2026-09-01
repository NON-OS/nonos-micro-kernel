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

/// Query buffer. A longer query is truncated rather than refused, so a paste
/// or a held key can never index past the buffer.
pub const QUERY_CAP: usize = 64;

#[derive(Clone, Copy)]
pub struct Palette {
    pub open: bool,
    pub query: [u8; QUERY_CAP],
    pub qlen: usize,
    pub sel: usize,
}

impl Default for Palette {
    fn default() -> Self {
        Self { open: false, query: [0u8; QUERY_CAP], qlen: 0, sel: 0 }
    }
}

impl Palette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(&mut self) {
        self.open = true;
        self.qlen = 0;
        self.sel = 0;
    }

    pub fn hide(&mut self) {
        self.open = false;
        self.qlen = 0;
        self.sel = 0;
    }

    pub fn needle(&self) -> &[u8] {
        &self.query[..self.qlen.min(QUERY_CAP)]
    }
}
