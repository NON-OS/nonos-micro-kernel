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

//! Asking the kernel whether another process holds a capability.
//!
//! The answer comes from the kernel's own capability table for that pid, so a
//! server deciding what a caller may do reads the attested record rather than
//! a name the caller could have chosen. The sender pid on a request is stamped
//! by the kernel; pairing it with this call gives an authorisation with no
//! string compare in it.

use crate::syscall::{call_raw, N_MK_CAP_CHECK};

/// One when `pid` holds every bit in `mask`, zero when it does not, negative
/// errno when the kernel refused to answer.
pub fn mk_cap_check(pid: u32, mask: u64) -> i64 {
    call_raw(N_MK_CAP_CHECK, [pid as u64, mask, 0, 0, 0, 0])
}
