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

//! Hosting code the kernel does not trust and does not understand.
//!
//! A foreign process holds no capabilities, so every NONOS syscall it makes
//! is refused at the contract gate. What it can do is issue a syscall
//! number this kernel has never heard of, which is exactly what a binary
//! built for another system does. Rather than answering `ENOSYS`, the
//! kernel hands that register frame to the userspace supervisor that
//! created the process and parks the caller until an answer comes back.
//!
//! The kernel copies six registers out and one value in. It does not read
//! them, does not know what they mean, and holds no table that could tell
//! it. Every syscall number, struct, path and errno belongs to the
//! supervisor, an ordinary attested capsule that can be replaced or
//! revoked without touching ring 0.

mod frame;
mod peer_chunk;
mod peer_copy;
mod peer_guard;
mod peer_map;
mod peer_protect;
mod registry;
mod spawn;
mod spawn_start;
mod trap;
mod trap_reply;
mod trap_table;
mod wait;

pub use frame::ForeignFrame;
pub use peer_copy::sys_peer_copy;
pub use peer_map::sys_peer_map;
pub use peer_protect::sys_peer_protect;
pub use registry::{clear, is_foreign, supervisor_of};
pub use spawn::sys_foreign_spawn;
pub use spawn_start::sys_foreign_start;
pub use trap::redirect;
pub use trap_reply::sys_foreign_reply;
pub use wait::sys_foreign_wait;
