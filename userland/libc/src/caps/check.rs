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

use crate::syscall::{call_raw, N_MK_CAP_CHECK};

/// Whether `pid` holds every bit in `mask`.
///
/// Ungated: any process may ask about any other, which discloses nothing the
/// process table does not already publish to everyone. It exists so a capsule
/// can ask before acting rather than discovering a refusal mid-operation, and
/// so a claim about what a process holds can be checked against the kernel's
/// own answer rather than a copy of it.
///
/// Returns 1 when held, 0 when not or when `pid` is unknown.
pub fn mk_cap_check(pid: u32, mask: u64) -> bool {
    call_raw(N_MK_CAP_CHECK, [pid as u64, mask, 0, 0, 0, 0]) == 1
}
