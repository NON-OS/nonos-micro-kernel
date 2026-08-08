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

//! The size bound every IPC path checks before it allocates.
//!
//! Kept in a file of its own so `mechanism_proofs` can include it and hold it
//! against `Nonos.Ipc.maxMessageSize`, which states the same number in Lean.
//! Two literals agreeing today is not the same as them being kept in step.

/// Largest payload an IPC message may carry. Checked by `sys_ipc_send`,
/// `sys_ipc_send_to_pid`, `sys_ipc_reply` and `sys_ipc_call` before any buffer
/// is allocated, so it bounds what one caller can make the kernel hold.
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024;
