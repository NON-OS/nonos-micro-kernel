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

use core::sync::atomic::{AtomicU64, Ordering};

/// Where the device tree said the PL031 is, or zero for a board without one.
static BASE: AtomicU64 = AtomicU64::new(0);

pub fn set_base(base: u64) {
    BASE.store(base, Ordering::Release);
}

pub(super) fn base() -> Option<u64> {
    match BASE.load(Ordering::Acquire) {
        0 => None,
        base => Some(base),
    }
}
