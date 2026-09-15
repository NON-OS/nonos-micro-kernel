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

//! Ask vfs_pool whether anything in the store has changed.
//!
//! The desktop and the package list each ran a full directory listing on every
//! clock tick, forever, and threw the identical answer away nearly every time.
//! A listing walks the tree, serialises every entry and copies it across an IPC
//! boundary, once a second, for the life of the session.
//!
//! This asks for eight bytes instead. When the number has not moved there is
//! nothing to list, and the expensive call is not made at all.

use alloc::vec;

use super::call::call;
use super::constants::{HDR_LEN, OP_GENERATION};

/// The store's mutation counter, or None when vfs_pool did not answer.
pub fn generation() -> Option<u64> {
    let mut rx = vec![0u8; HDR_LEN + 16];
    let total = call(OP_GENERATION, &[], &mut rx)?;
    if total < HDR_LEN + 12 {
        return None;
    }
    let off = HDR_LEN + 4;
    let mut n = [0u8; 8];
    n.copy_from_slice(&rx[off..off + 8]);
    Some(u64::from_le_bytes(n))
}
