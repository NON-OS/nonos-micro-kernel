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

// The NONOS file-descriptor emulation layer. NONOS has no kernel file
// descriptors: I/O is IPC to the net.sockets and vfs service capsules,
// addressed by opaque `u32` handles. `std::os::fd`, and every fd-based crate
// above it, assume a small-integer `RawFd`; this module bridges the two with a
// process-local table that maps each handed-out `RawFd` to the service handle
// backing it, so a descriptor can be duplicated and closed by dispatching to
// the right IPC call.

mod desc;
mod ipc;
mod table;

pub mod stdio_raw;

pub use desc::FileDesc;
pub use table::{Backing, close_fd, dup_fd, get, install, register_socket, socket_handle};
