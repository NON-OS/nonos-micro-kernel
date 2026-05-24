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

//! Leaf descriptor returned by the page-table walker. Carries the
//! raw PTE, the physical base of the leaf page, the offset within
//! the leaf where the requested VA lands, and the leaf page size
//! (4 KiB, 2 MiB, or 1 GiB).

#[derive(Clone, Copy)]
pub(crate) struct UserLeaf {
    pub entry: u64,
    pub phys_base: u64,
    pub offset: u64,
    pub size: u64,
}

impl UserLeaf {
    pub(crate) fn bytes_remaining_in_page(&self) -> u64 {
        self.size - self.offset
    }
}
