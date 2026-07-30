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

//! Page-table entries, built and read through the architecture that defines
//! them.
//!
//! The shape of the walk is shared. Both architectures this kernel targets use
//! four levels of nine-bit index over a 4 KiB page, so the manager above can
//! index, descend and allocate tables without knowing whose tables they are.
//! What is not shared is what a single entry means: x86_64 marks a mapping
//! writable by setting a bit while aarch64 marks it read-only by setting one,
//! x86_64 flags a huge page by setting bit 7 while aarch64 flags a block by
//! leaving bit 1 clear, and the two disagree about which bits hold the output
//! address.
//!
//! So every entry the manager writes goes through [`leaf`] or [`table`], and
//! every entry it reads goes through the predicates here. A raw `|` against a
//! flag constant would be right on one architecture and quietly wrong on the
//! other.

pub mod flags;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64/mod.rs"]
mod backend;
#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod backend;

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[path = "unsupported.rs"]
mod backend;

pub use backend::{
    address, is_block, is_present, is_user, is_writable, leaf, table, table_grants_user, ADDR_MASK,
};
