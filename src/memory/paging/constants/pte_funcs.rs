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

//! Reading a live page-table entry.
//!
//! The bit that answers each of these differs per architecture, so all four
//! go through `arch::paging::descriptor` rather than masking here. The names
//! stay as the manager has always spelled them.

use crate::arch::paging::descriptor;

#[inline]
pub const fn pte_is_present(pte: u64) -> bool {
    descriptor::is_present(pte)
}

/// True when the entry maps a block rather than pointing at another table.
/// Only ask at a level where a block is legal.
#[inline]
pub const fn pte_is_huge(pte: u64) -> bool {
    descriptor::is_block(pte)
}

#[inline]
pub const fn pte_address(pte: u64) -> u64 {
    descriptor::address(pte)
}

#[inline]
pub const fn pte_is_writable(pte: u64) -> bool {
    descriptor::is_writable(pte)
}
