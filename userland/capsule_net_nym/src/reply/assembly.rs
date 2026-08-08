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

/// Fragments of one message, held until all of them have arrived.
pub struct Assembly {
    pub(super) set_id: i32,
    pub(super) total: u8,
    pub(super) pieces: Vec<Option<Vec<u8>>>,
    pub(super) held: u8,
}

impl Assembly {
    pub(super) fn new(set_id: i32, total: u8) -> Self {
        let mut pieces = Vec::with_capacity(total as usize);
        pieces.resize_with(total as usize, || None);
        Self { set_id, total, pieces, held: 0 }
    }

    /// Whether this holds fragments of `set_id`.
    pub fn holds(&self, set_id: i32) -> bool {
        self.set_id == set_id
    }
}
