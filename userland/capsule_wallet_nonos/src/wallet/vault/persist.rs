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

//! The step that makes a write outlive the power.
//!
//! Without it the file is in the RAM filesystem and nowhere else. The block
//! store is what the seeder reads at boot, so a vault that was written but
//! not persisted is a vault that quietly does not exist.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use super::vfs::{call, OP_STORE_PERSIST};

pub(super) fn persist(path: &[u8]) -> bool {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(5 + path.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(path.len() as u8);
    body.extend_from_slice(path);
    let mut rx = vec![0u8; 64];
    call(OP_STORE_PERSIST, &body, &mut rx).worked()
}
