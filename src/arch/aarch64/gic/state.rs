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

static DIST_BASE: AtomicU64 = AtomicU64::new(0);
static REDIST_BASE: AtomicU64 = AtomicU64::new(0);

pub(super) fn set_bases(dist: u64, redist: u64) {
    DIST_BASE.store(dist, Ordering::Release);
    REDIST_BASE.store(redist, Ordering::Release);
}

pub(super) fn dist_base() -> u64 {
    DIST_BASE.load(Ordering::Acquire)
}

pub(super) fn redist_base() -> u64 {
    REDIST_BASE.load(Ordering::Acquire)
}
