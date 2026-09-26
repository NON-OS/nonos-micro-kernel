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

//! Which limit each resource number reports.

use crate::linux::file::MAX_FDS;

/// `struct rlimit` is a soft limit then a hard one, both 64-bit.
pub(super) const RLIMIT: usize = 16;

const RLIMIT_STACK: u64 = 3;
const RLIMIT_NOFILE: u64 = 7;
const RLIMIT_AS: u64 = 9;

/// What a guest's stack is given, from the loader that maps it.
const STACK_BYTES: u64 = 1 << 20;

/// The top of the guest's own half, which is the most address space one
/// can hold however it asks.
const ADDRESS_SPACE: u64 = 0x0000_7FFF_F000;

/// Unlimited, as Linux spells it.
const INFINITY: u64 = u64::MAX;

pub(super) fn limit_for(resource: u64) -> Option<(u64, u64)> {
    match resource {
        RLIMIT_STACK => Some((STACK_BYTES, STACK_BYTES)),
        RLIMIT_NOFILE => Some((MAX_FDS as u64, MAX_FDS as u64)),
        RLIMIT_AS => Some((ADDRESS_SPACE, ADDRESS_SPACE)),
        /*
         * Everything else this personality does not bound at all, and saying
         * so is truthful: there is no ceiling to report.
         */
        _ => Some((INFINITY, INFINITY)),
    }
}

