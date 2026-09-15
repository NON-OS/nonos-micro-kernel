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

//! Whether this window holds what its manifest says, asked two ways.

use nonos_libc::mk_cap_check;

use super::super::caps::MASK;
use super::types::Verdict;

/// Two independent kernel paths must agree: the mask in the process table,
/// read as everyone reads it, and the kernel's own MkCapCheck over the same
/// bits. One path can be wrong on its own; two disagreeing is a kernel fault
/// surfacing where a person can see it, which is what this screen is for.
///
/// A window not present in its own process table has nothing to compare and
/// reports unknown rather than pass.
pub fn own_mask_verdict(me: u32, from_table: u64) -> Verdict {
    if from_table == 0 {
        return Verdict::Unknown;
    }
    Verdict::from_bool(from_table == MASK && mk_cap_check(me, MASK))
}
