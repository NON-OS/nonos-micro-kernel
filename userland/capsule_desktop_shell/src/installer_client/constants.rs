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

//! Wire constants for the installer protocol.

pub(super) const SERVICE: &[u8] = b"installer";
pub(super) const HDR_LEN: usize = 8;
pub(super) const SEQ: u32 = 1;

pub(super) const OP_LIST_INSTALLED: u16 = 5;

pub(super) const TIMEOUT_MS: u64 = 300;

/// Reply buffer size, the same budget the desktop gives a vfs listing. Anything
/// past it is simply not shown rather than risking a large per-tick allocation.
pub(super) const REPLY_CAP: usize = 16384;

/// Longest capsule-store base name the store contract allows.
pub(super) const MAX_NAME: usize = 64;

/// Upper bound on decoded names, so a hostile reply cannot turn one buffer into
/// an unbounded number of allocations.
pub(super) const MAX_ENTRIES: usize = 256;
