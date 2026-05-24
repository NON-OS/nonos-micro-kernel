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

use super::constants::DEFAULT_BASE;
use super::device::Clint;

static CLINT_BASE: AtomicU64 = AtomicU64::new(DEFAULT_BASE);

pub fn set_clint_base(base: u64) {
    if base != 0 {
        CLINT_BASE.store(base, Ordering::Release);
    }
}

pub fn clint_base() -> u64 {
    CLINT_BASE.load(Ordering::Acquire)
}

pub(super) fn current_clint() -> Clint {
    Clint::new(clint_base())
}
