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

//! What the shared paging manager asks a mapping to be.
//!
//! These bit positions are the x86_64 ones, which makes the translation on
//! that architecture the identity and costs nothing. They are not hardware
//! bits: they are the vocabulary the manager passes around, and every
//! architecture's backend reads them and emits whatever its own descriptors
//! need. Nothing outside `arch::paging::descriptor` may write them into a page
//! table.

/// There is a mapping here.
pub const PRESENT: u64 = 1 << 0;
/// Stores are permitted. Its absence means read-only, which some
/// architectures encode as a set bit rather than a clear one.
pub const WRITABLE: u64 = 1 << 1;
/// Reachable from EL0 / ring 3.
pub const USER: u64 = 1 << 2;
pub const WRITE_THROUGH: u64 = 1 << 3;
/// Uncached. Set for device memory.
pub const NO_CACHE: u64 = 1 << 4;
pub const ACCESSED: u64 = 1 << 5;
pub const DIRTY: u64 = 1 << 6;
/// This leaf covers a whole block rather than one page, so it sits above the
/// last translation level.
pub const HUGE: u64 = 1 << 7;
/// Survives an address-space switch.
pub const GLOBAL: u64 = 1 << 8;
/// Instruction fetch is denied.
pub const NO_EXECUTE: u64 = 1 << 63;
