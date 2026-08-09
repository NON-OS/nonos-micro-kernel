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
use crate::protocol::{OP_FLUSH, OP_WRITE_BLOCKS};
// The sender pid is parsed by the kernel from the message envelope it stamps
// itself (`proc.<pid>`), so a capsule cannot forge it, and pid 0 is never
// handed to a process. Only the kernel-internal client enqueues without that
// prefix and so arrives as 0. Mutating operations are reserved for it: the
// package store at LBA 0 is read back as trusted input on the next boot.
pub fn permits(op: u16, sender_pid: u32) -> bool {
    match op {
        OP_WRITE_BLOCKS | OP_FLUSH => sender_pid == 0,
        _ => true,
    }
}
