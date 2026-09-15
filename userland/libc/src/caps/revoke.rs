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

use crate::syscall::{call_raw, N_MK_CAP_REVOKE};

/// Withdraw every bit in `mask` from `pid`.
///
/// Admin only, as for grant. A capsule may not narrow its own authority
/// through this call; the mask a capsule runs with is what its signed manifest
/// declared, and shrinking it at runtime would make the manifest a ceiling
/// rather than the truth.
///
/// Returns 0, or a negative errno.
pub fn mk_cap_revoke(pid: u32, mask: u64) -> i64 {
    call_raw(N_MK_CAP_REVOKE, [pid as u64, mask, 0, 0, 0, 0])
}
