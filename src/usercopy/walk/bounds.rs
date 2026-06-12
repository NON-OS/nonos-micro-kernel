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

//! Directmap bounds for the page walk. Every table page and leaf
//! frame is read through the directmap, so a physical address past
//! its window would turn a corrupt PTE into a wild kernel
//! dereference. Reject the walk instead.

use super::leaf::UserLeaf;
use crate::memory::layout::{DIRECTMAP_BASE, DIRECTMAP_SIZE};
use crate::usercopy::error::UsercopyError;

pub(super) fn directmap_of(phys: u64) -> Result<u64, UsercopyError> {
    if phys >= DIRECTMAP_SIZE {
        return Err(UsercopyError::PageTableCorrupt);
    }
    Ok(DIRECTMAP_BASE + phys)
}

pub(super) fn leaf_in_directmap(leaf: UserLeaf) -> Result<UserLeaf, UsercopyError> {
    if leaf.phys_base >= DIRECTMAP_SIZE - (leaf.size - 1) {
        return Err(UsercopyError::PageTableCorrupt);
    }
    Ok(leaf)
}
