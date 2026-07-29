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

//! What a mapping is asked to be.
//!
//! These describe intent, not hardware. `arch::paging::descriptor` owns the
//! vocabulary and every backend translates it into its own descriptor bits, so
//! nothing here may be written into a page table directly: build entries with
//! `descriptor::leaf` and `descriptor::table`.

use crate::arch::paging::descriptor::flags;

pub const PTE_PRESENT: u64 = flags::PRESENT;
pub const PTE_WRITABLE: u64 = flags::WRITABLE;
pub const PTE_USER: u64 = flags::USER;
pub const PTE_WRITE_THROUGH: u64 = flags::WRITE_THROUGH;
pub const PTE_CACHE_DISABLE: u64 = flags::NO_CACHE;
pub const PTE_ACCESSED: u64 = flags::ACCESSED;
pub const PTE_DIRTY: u64 = flags::DIRTY;
pub const PTE_HUGE_PAGE: u64 = flags::HUGE;
pub const PTE_GLOBAL: u64 = flags::GLOBAL;
pub const PTE_NO_EXECUTE: u64 = flags::NO_EXECUTE;

/// The output-address bits of a live entry on this architecture. Prefer
/// `pte_address`, which is the same thing without the caller doing the mask.
pub const PTE_ADDR_MASK: u64 = crate::arch::paging::descriptor::ADDR_MASK;
pub const PTE_FLAGS_MASK: u64 = !PTE_ADDR_MASK;
