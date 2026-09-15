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

use crate::syscall::{call_raw, N_MK_CAP_GRANT};

/// Extend `pid` by every bit in `mask`.
///
/// The kernel refuses unless the caller holds Admin and also holds every bit
/// it is handing on: an administrator cannot manufacture authority it does not
/// itself possess. Today only init holds Admin, so from any shipped capsule
/// this returns a refusal, which is the correct answer and is now an answer
/// rather than an absence.
///
/// Returns 0, or a negative errno.
pub fn mk_cap_grant(pid: u32, mask: u64) -> i64 {
    call_raw(N_MK_CAP_GRANT, [pid as u64, mask, 0, 0, 0, 0])
}
