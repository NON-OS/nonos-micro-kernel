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

//! The write-authority decision, pure so the host proofs can walk every case.

use crate::protocol::{OP_FLUSH, OP_WRITE_BLOCKS};

/// The sender pid is stamped by the kernel and pid 0 is never handed to a
/// process, so it marks the kernel-internal client and nothing else. Any
/// other sender writes only when the kernel says it holds StoreWrite.
pub fn allows(op: u16, sender_pid: u32, sender_may_write: bool) -> bool {
    match op {
        OP_WRITE_BLOCKS | OP_FLUSH => sender_pid == 0 || sender_may_write,
        _ => true,
    }
}
