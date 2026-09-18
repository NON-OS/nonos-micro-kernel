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


//! Reaching into a guest this process supervises: its pages, their
//! protection, and the bytes in them. Every one of these is refused by
//! the kernel unless the caller created the process it names.

use crate::syscall::{call_raw, N_MK_PEER_COPY, N_MK_PEER_MAP, N_MK_PEER_PROTECT};

/// Pages of a guest may be written, and may be executed.
pub const PEER_PROT_WRITE: u64 = 1 << 0;
pub const PEER_PROT_EXEC: u64 = 1 << 1;

/// Back a span of a guest's address space with fresh zeroed frames.
pub fn mk_peer_map(pid: u32, addr: u64, len: u64, prot: u64) -> i64 {
    call_raw(N_MK_PEER_MAP, [pid as u64, addr, len, prot, 0, 0])
}

/// Set the protection of pages a guest already has. Every page in the
/// span must be mapped: the kernel refuses a hole rather than filling it,
/// because a caller asking for execute on a range it has not written is
/// not asking for what it thinks.
pub fn mk_peer_protect(pid: u32, addr: u64, len: u64, prot: u64) -> i64 {
    call_raw(N_MK_PEER_PROTECT, [pid as u64, addr, len, prot, 0, 0])
}

/// Copy into a guest this process supervises.
pub fn mk_peer_write(pid: u32, guest_addr: u64, src: &[u8]) -> i64 {
    let args = [pid as u64, guest_addr, src.as_ptr() as u64, src.len() as u64, 1, 0];
    call_raw(N_MK_PEER_COPY, args)
}

/// Copy out of a guest this process supervises.
pub fn mk_peer_read(pid: u32, guest_addr: u64, dst: &mut [u8]) -> i64 {
    let args = [pid as u64, guest_addr, dst.as_mut_ptr() as u64, dst.len() as u64, 0, 0];
    call_raw(N_MK_PEER_COPY, args)
}
